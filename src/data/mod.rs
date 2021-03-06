/*
 * mod.rs
 * Module header for mcisa's data module
 * Created by Sarah Davis
 * Created on 2/27/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// module exports
mod mov_data;
pub use mov_data::MOVData;
mod add_data;
pub use add_data::ADDData;
mod sub_data;
pub use sub_data::SUBData;
mod mul_data;
pub use mul_data::MULData;

// end of file
