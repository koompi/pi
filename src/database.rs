use super::{Application, BuildFile};
use serde::{Deserialize, Serialize};

pub struct LocalDB {
    applications: Vec<Application>,
}

pub struct OnlineDB {
    applications: Vec<Application>,
}

impl RegistrDB {
    pub fn find_by_name(&self, name: String) -> Option<BuildFile> {
        self.pkgbuilds
            .iter()
            .find(|build_file| build_file.metadata.name == name)
    }
}

pub struct RegistrDB {
    applications: Vec<Application>,
}

impl RegistrDB {
    pub fn find_by_name(&self, name: String) -> Option<BuildFile> {
        self.pkgbuilds
            .iter()
            .find(|build_file| build_file.metadata.name == name)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BuildFileDB {
    pkgbuilds: Vec<BuildFile>,
}

impl BuildFileDB {
    pub fn find_by_name(&self, name: String) -> Option<BuildFile> {
        self.pkgbuilds
            .iter()
            .find(|build_file| build_file.metadata.name == name)
    }

    pub fn find_dependencies(&self, name: String, initial: Option<Vec<String>>) -> Vec<String> {
        let mut data: Vec<String> = Vec::new();
        // get all run time dependencies
        if let Some(build_file) = self.find_by_name(name) {
            if let Some(dependencies) = build_file.dependencies {
                if let Some(run_dependencies) = dependencies.run_dependencies {
                    if !run_dependencies.is_empty() {
                        // data.append(run_dependencies)
                        run_dependencies.iter().for_each(f)
                    }
                }
            }
        }
    }
}

/* Required some Rust code to flatten an array of arbitrarily nested arrays of integers into a flat array of
 * integers. e.g. [[1,2,[3]],4] -> [1,2,3,4].
 */

/* Model of an arbitrarily nested array of arrays, since native arrays in Rust won't allow that.
 * It is a generic type so that it supports all native numeric types.
 */
enum ArbitrarilyNestedArray<T> {
    Array(Vec<ArbitrarilyNestedArray<T>>),
    Integer(T),
}

/* This will flatten an array of any type implementing the 'Copy' trait, so all numeric
 * types should be covered
 */
fn flatten<T: Copy>(arr: &ArbitrarilyNestedArray<T>) -> Vec<T> {
    match arr {
        // base case
        &ArbitrarilyNestedArray::Integer(x) => vec![x],
        // recursive case, flatten each member and concat all of them
        &ArbitrarilyNestedArray::Array(ref v) => {
            let mut flat_v = vec![];
            for a in v {
                let flat_a = flatten(a);
                for e in flat_a {
                    flat_v.push(e);
                }
            }
            flat_v
        }
    }
}

/* Basic tests checking some general cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_array() {
        let empty_arr: ArbitrarilyNestedArray<u64> = ArbitrarilyNestedArray::Array(vec![]);
        assert!(flatten(&empty_arr).is_empty());
    }

    #[test]
    fn flat_array() {
        let empty_arr = ArbitrarilyNestedArray::Array(vec![
            ArbitrarilyNestedArray::Integer(1),
            ArbitrarilyNestedArray::Integer(2),
            ArbitrarilyNestedArray::Integer(3),
        ]);
        assert_eq!(flatten(&empty_arr), vec!(1, 2, 3));
    }

    #[test]
    fn deeply_nested_array() {
        let empty_arr = ArbitrarilyNestedArray::Array(vec![
            ArbitrarilyNestedArray::Array(vec![
                ArbitrarilyNestedArray::Integer(1),
                ArbitrarilyNestedArray::Integer(2),
                ArbitrarilyNestedArray::Array(vec![
                    ArbitrarilyNestedArray::Integer(3),
                    ArbitrarilyNestedArray::Array(vec![]),
                ]),
            ]),
            ArbitrarilyNestedArray::Integer(4),
        ]);
        assert_eq!(flatten(&empty_arr), vec!(1, 2, 3, 4));
    }
}
