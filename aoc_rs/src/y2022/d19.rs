use crate::{aocerror, AOCError, ProblemPart};

use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Resources {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
}

fn clamped_sub(a: u64, b: u64) -> u64 {
    a.checked_sub(b).unwrap_or(0)
}

fn clamped_divish(a: u64, b: u64) -> Option<u64> {
    if a == 0 {
        Some(0)
    } else {
        let remainder_addition = if b != 0 && a % b != 0 { 1 } else { 0 };
        a.checked_div(b).map(|x| x + remainder_addition)
    }
}

impl Resources {
    fn checked_sub(&self, rhs: &Resources) -> Option<Resources> {
        match (
            self.ore.checked_sub(rhs.ore),
            self.clay.checked_sub(rhs.clay),
            self.obsidian.checked_sub(rhs.obsidian),
            self.geode.checked_sub(rhs.geode),
        ) {
            (Some(o), Some(c), Some(ob), Some(g)) => Some(Resources {
                ore: o,
                clay: c,
                obsidian: ob,
                geode: g,
            }),
            _ => None,
        }
    }

    fn clamped_sub(&self, rhs: &Resources) -> Resources {
        Resources {
            ore: clamped_sub(self.ore, rhs.ore),
            clay: clamped_sub(self.clay, rhs.clay),
            obsidian: clamped_sub(self.obsidian, rhs.obsidian),
            geode: clamped_sub(self.geode, rhs.geode),
        }
    }

    fn time_to_make(&self, income: &Resources) -> Option<u64> {
        let quotients = [
            clamped_divish(self.ore, income.ore),
            clamped_divish(self.clay, income.clay),
            clamped_divish(self.obsidian, income.obsidian),
            clamped_divish(self.geode, income.geode),
        ];
        quotients
            .iter()
            .fold(Some(0), |lhs, rhs| lhs.and_then(|l| rhs.map(|r| l.max(r))))
    }

    fn add(&self, rhs: &Resources) -> Resources {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }

    fn times(&self, scalar: u64) -> Resources {
        Resources {
            ore: self.ore * scalar,
            clay: self.clay * scalar,
            obsidian: self.obsidian * scalar,
            geode: self.geode * scalar,
        }
    }

    fn zero() -> Resources {
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn new(ore: u64, clay: u64, obsidian: u64, geode: u64) -> Resources {
        Resources {
            ore: ore,
            clay: clay,
            obsidian: obsidian,
            geode: geode,
        }
    }

    fn pointwise_max(&self, rhs: &Resources) -> Resources {
        Resources::new(
            self.ore.max(rhs.ore),
            self.clay.max(rhs.clay),
            self.obsidian.max(rhs.obsidian),
            self.geode.max(rhs.geode),
        )
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u64,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

fn parse_ore_and_resource(s: &str) -> Result<(u64, Option<(u64, &str)>), AOCError> {
    let mut words = s.trim().split(' ').skip(4);
    let ore_cost = words
        .next()
        .ok_or(aocerror!("not enough words in string: {}", s))
        .and_then(|s| s.parse::<u64>().map_err(AOCError::from))?;
    let mut rest = words.skip(2);
    if let Some(other_resource_str) = rest.next() {
        let other_resource_cost = other_resource_str.parse::<u64>()?;
        let other_resource = rest
            .next()
            .ok_or(aocerror!("not enough words in string: {}", s))?;
        Ok((ore_cost, Some((other_resource_cost, other_resource))))
    } else {
        Ok((ore_cost, None))
    }
}

impl FromStr for Blueprint {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: u64 = s.split(' ').skip(1).next().map_or(
            Err(aocerror!("can't parse blueprint: {}", s)),
            |n| {
                n.trim_end_matches(':')
                    .parse::<u64>()
                    .map_err(AOCError::from)
            },
        )?;
        let mut resource_costs = s
            .split_once(':')
            .ok_or(aocerror!("can't parse blueprint: {}", s))?
            .1
            .split('.')
            .map(|s| parse_ore_and_resource(s));
        let ore_costs = resource_costs
            .next()
            .ok_or(aocerror!("can't parse blueprint: {}", s))??;
        if ore_costs.1.is_some() {
            // return Err(AOCError {});
            return Err(aocerror!(
                "unexpected result when parsing ore costs: {:?}, {}",
                ore_costs.1,
                s
            ));
        }
        let clay_costs = resource_costs
            .next()
            .ok_or(aocerror!("can't parse blueprint: {}", s))??;
        if clay_costs.1.is_some() {
            return Err(aocerror!(
                "unexpected result when parsing clay costs: {:?}, {}",
                clay_costs.1,
                s
            ));
        }
        let obsidian_costs = resource_costs
            .next()
            .ok_or(aocerror!("can't parse blueprint: {}", s))??;
        let geode_costs = resource_costs
            .next()
            .ok_or(aocerror!("can't parse blueprint: {}", s))??;
        match (obsidian_costs, geode_costs) {
            (
                (obs_robot_ore, Some((obs_robot_clay, "clay"))),
                (geode_robot_ore, Some((geode_robot_obs, "obsidian"))),
            ) => Ok(Blueprint {
                id: id,
                ore_robot_cost: Resources::new(ore_costs.0, 0, 0, 0),
                clay_robot_cost: Resources::new(clay_costs.0, 0, 0, 0),
                obsidian_robot_cost: Resources::new(obs_robot_ore, obs_robot_clay, 0, 0),
                geode_robot_cost: Resources::new(geode_robot_ore, 0, geode_robot_obs, 0),
            }),
            _ => Err(aocerror!("can't parse blueprint: {}", s)),
        }
    }
}

impl Blueprint {
    fn action_states(&self, total_time: u64, initial_income: Resources) -> BlueprintActionIterator {
        let mut states = VecDeque::new();
        states.push_back(SearchState {
            remaining_time: total_time,
            current_resources: Resources::zero(),
            income: initial_income,
        });
        let mut max_useful_income = self
            .ore_robot_cost
            .pointwise_max(&self.clay_robot_cost)
            .pointwise_max(&self.obsidian_robot_cost)
            .pointwise_max(&self.geode_robot_cost);
        max_useful_income.geode = u64::MAX;
        BlueprintActionIterator {
            blueprint: &self,
            states: states,
            max_useful_income: max_useful_income,
        }
    }
}

fn build_robot_when_possible(
    remaining_time: u64,
    robot_cost: &Resources,
    income: &Resources,
    current: &Resources,
) -> Option<(u64, Resources)> {
    if remaining_time == 0 {
        return None;
    }
    if let Some(turns_to_wait) = robot_cost.clamped_sub(&current).time_to_make(&income) {
        let time_spent_to_build = turns_to_wait + 1;
        let time_left_after_build = remaining_time.checked_sub(time_spent_to_build);
        time_left_after_build.map(|time_left| {
            (
                time_left,
                current
                    .add(&income.times(time_spent_to_build))
                    .checked_sub(&robot_cost)
                    .unwrap(),
            )
        })
    } else {
        None
    }
}

#[derive(Debug)]
struct SearchState {
    remaining_time: u64,
    current_resources: Resources,
    income: Resources,
}

struct BlueprintActionIterator<'a> {
    blueprint: &'a Blueprint,
    states: VecDeque<SearchState>,
    max_useful_income: Resources,
}

struct UsefulRobotIterator<'a> {
    blueprint: &'a Blueprint,
    max_useful_income: &'a Resources,
    current_state: &'a SearchState,
    current_resource: usize,
}

impl<'a> Iterator for UsefulRobotIterator<'a> {
    type Item = (&'a Resources, Resources);

    // best heuristic i read on the reddit thread:
    // relax to a problem which is solvable w/ a greedy algorithm.
    // relaxations:
    //   1. ore costs of all robots set to 0.
    //   2. any number of robots can be built each turn
    // now ore is useless and clay robots are free.

    // if you can build a robot on a turn, there's no reason not to
    // build it because robots cost disjoint resources. so greedy
    // search: each turn build any robot you can.

    // now use this relaxation to upper bound the best possible value
    // you can get from a given SearchState. if you can't beat the
    // current record with the greedy relaxation then there is no
    // point expanding the state.

    // pair this with DFS instead of BFS, and some heuristic to chose
    // a better node.  or some kind of priority queue:
    // std::collections::BinaryHeap
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_resource >= 4 {
            return None;
        }
        let original_resource = self.current_resource;
        self.current_resource += 1;
        match original_resource {
            0 => {
                if self.current_state.income.ore < self.max_useful_income.ore {
                    Some((&self.blueprint.ore_robot_cost, Resources::new(1, 0, 0, 0)))
                } else {
                    self.next()
                }
            }
            1 => {
                let have_geode = self.current_state.income.geode > 0;
                // this have_geode check is a possibly unsound heuristic.
                if self.current_state.income.clay < self.max_useful_income.clay && !have_geode {
                    Some((&self.blueprint.clay_robot_cost, Resources::new(0, 1, 0, 0)))
                } else {
                    self.next()
                }
            }
            2 => {
                if self.current_state.income.obsidian < self.max_useful_income.obsidian {
                    Some((
                        &self.blueprint.obsidian_robot_cost,
                        Resources::new(0, 0, 1, 0),
                    ))
                } else {
                    self.next()
                }
            }
            3 => Some((&self.blueprint.geode_robot_cost, Resources::new(0, 0, 0, 1))),
            _ => None,
        }
    }
}

fn get_useful_robots<'a>(
    blueprint: &'a Blueprint,
    max_useful_income: &'a Resources,
    current_state: &'a SearchState,
) -> impl Iterator<Item = (&'a Resources, Resources)> + 'a {
    UsefulRobotIterator {
        blueprint: blueprint,
        max_useful_income: max_useful_income,
        current_state: current_state,
        current_resource: 0,
    }
}

impl<'a> Iterator for BlueprintActionIterator<'a> {
    type Item = SearchState;

    fn next(&mut self) -> Option<Self::Item> {
        match self.states.pop_front() {
            None => None,
            Some(state) => {
                for (cost, additional_income) in
                    get_useful_robots(self.blueprint, &self.max_useful_income, &state)
                {
                    if let Some((rem_time, new_resources)) = build_robot_when_possible(
                        state.remaining_time,
                        &cost,
                        &state.income,
                        &state.current_resources,
                    ) {
                        let ss = SearchState {
                            remaining_time: rem_time,
                            current_resources: new_resources,
                            income: state.income.add(&additional_income),
                        };
                        self.states.push_back(ss);
                    }
                }
                // i think there's a bug here - if we don't create any
                // more robots, we should wait out the remaining time
                // to 0. imagine a blueprint where each robot is very
                // expensive, so you can't afford to make a robot
                // every turn. the last turn of the optimal paths
                // might not make a robot.
                Some(state)
            }
        }
    }
}

fn blueprint_max_geodes(blueprint: &Blueprint, total_time: u64) -> u64 {
    let mut max_geode = 0;
    for state in blueprint.action_states(total_time, Resources::new(1, 0, 0, 0)) {
        max_geode = max_geode.max(state.current_resources.geode);
    }
    max_geode
}

fn parse_problem<B: io::BufRead>(bread: B) -> Result<Vec<Blueprint>, AOCError> {
    bread.lines().map(|s| s?.parse::<Blueprint>()).collect()
}

fn problem1(blueprints: &[Blueprint]) -> u64 {
    let mut sum = 0;
    for blueprint in blueprints {
        sum += blueprint_max_geodes(blueprint, 24) * blueprint.id;
    }
    sum
}

fn problem2(blueprints: &[Blueprint]) -> u64 {
    let mut prod = 1;
    for blueprint in blueprints.iter().take(3) {
        prod *= blueprint_max_geodes(blueprint, 32);
    }
    prod
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let blueprints = parse_problem(br)?;
    let result = match part {
        ProblemPart::P1 => problem1(&blueprints),
        ProblemPart::P2 => problem2(&blueprints),
    };
    println!("result: {}", result);
    Ok(())
}
