// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<E: Environment> FromField for Address<E> {
    type Field = Field<E>;

    /// Initializes a new address by recovering the **x-coordinate** of an affine group from a field element.
    fn from_field(field: &Self::Field) -> Result<Self> {
        Ok(Address::new(Group::from_field(field)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network_environment::Console;

    type CurrentEnvironment = Console;

    const ITERATIONS: u64 = 10_000;

    fn check_from_field() -> Result<()> {
        let mut rng = TestRng::default();

        for _ in 0..ITERATIONS {
            // Sample a random value.
            let expected = Address::<CurrentEnvironment>::new(Uniform::rand(&mut rng));
            let candidate = Address::<CurrentEnvironment>::from_field(&expected.to_field()?)?;
            assert_eq!(expected, candidate);
        }
        Ok(())
    }

    #[test]
    fn test_from_field() -> Result<()> {
        check_from_field()
    }
}
