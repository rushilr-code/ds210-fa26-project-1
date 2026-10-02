//! Running many games and reporting what happened.

use crate::secret_keeper::Dealer;
use crate::strategies::{Approach, bad, binary, clever, jump, linear, lucky, random};

/// What a run of games revealed about a strategy.
#[derive(Debug)]
pub struct Results {
    /// The fewest questions any single game needed.
    pub best: u32,
    /// The average across every game.
    pub mean: f64,
    /// The most any single game needed.
    pub worst: u32,
    /// The standard deviation: how spread out the games were around the mean.
    pub std_dev: f64,
}

impl Results {
    /// Is this strategy better than `other`?
    /// **You decide what better means**, using any of the four fields, or several of them.
    /// There is no single right answer (though there are wrong ones), so make a choice
    /// you can defend.
    pub fn is_better_than(&self, other: &Results) -> bool {
        // I define "better" as needing fewer questions on average.
        // If the averages are equal, use the worst case as a tie-breaker,
        // then the standard deviation.
        if self.mean < other.mean {
            true
        } else if self.mean > other.mean {
            false
        } else if self.worst < other.worst {
            true
        } else if self.worst > other.worst {
            false
        } else {
            self.std_dev < other.std_dev
        }
    }
}


/// Play `rounds` games of `approach` on the range `[min, max)` and report how many
/// questions they took. Every game is dealt from one `Dealer`, so no secret
/// repeats until the whole range has been used.
pub fn measure(approach: Approach, min: u32, max: u32, rounds: u32) -> Results {
    if rounds == 0 {
        return Results {
            best: 0,
            mean: 0.0,
            worst: 0,
            std_dev: 0.0,
        };
    }

    let strategy: fn(&mut crate::secret_keeper::SecretKeeper, u32, u32) -> u32 =
        match approach {
            Approach::Bad => bad,
            Approach::Random => random,
            Approach::Linear => linear,
            Approach::Binary => binary,
            Approach::Jump => jump,
            Approach::Lucky => lucky,
            Approach::Clever => clever,
        };

    let mut dealer = Dealer::new(min, max);

    let mut best = u32::MAX;
    let mut worst = 0;
    let mut total = 0.0;
    let mut total_squared = 0.0;

    for _ in 0..rounds {
        let mut keeper = dealer.deal();

        strategy(&mut keeper, min, max);

        let questions = keeper.questions_asked();

        if questions < best {
            best = questions;
        }

        if questions > worst {
            worst = questions;
        }

        let questions_f64 = questions as f64;
        total += questions_f64;
        total_squared += questions_f64 * questions_f64;
    }

    let mean = total / rounds as f64;

    let variance =
        (total_squared / rounds as f64) - (mean * mean);

    let std_dev = variance.max(0.0).sqrt();

    Results {
        best,
        mean,
        worst,
        std_dev,
    }
}
