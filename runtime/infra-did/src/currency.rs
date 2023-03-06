// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

#![allow(non_upper_case_globals)]
use common_primitives::types::Balance;

pub const IDID: Balance = 1_000_000_000_000_000_000; // 18 decimal
pub const cIDID: Balance = IDID / 100; // 16 decimal, cent-IDID
pub const mIDID: Balance = IDID / 1_000; //15 decimal, milli-IDID
pub const uIDID: Balance = IDID / 1_000_000; // 12 decimal, micro-IDID

pub const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 15 * mIDID + (bytes as Balance) * 6 * mIDID // TODO: revisit the storage cost here
}
