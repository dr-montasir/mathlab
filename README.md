[<img alt="github" src="https://img.shields.io/badge/github-dr%20montasir%20/%20mathlab-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/dr-montasir/mathlab)[<img alt="crates.io" src="https://img.shields.io/crates/v/mathlab.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/mathlab)[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-mathlab-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/mathlab)[<img alt="license" src="https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="22">](https://choosealicense.com/licenses/apache-2.0)

# MathLab

> A Powerful Math Library for Rust

## Install

Run the following Cargo command in your project directory:

```shell
cargo add mathlab
```

or

Add the following line to your `Cargo.toml` file with the specified version:

```toml
mathlab = "MAJOR.MINOR.PATCH"
```

## Changelog

[![github](https://img.shields.io/badge/github-%20changelog-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dr-montasir/mathlab/blob/master/CHANGELOG.md)

https://github.com/dr-montasir/mathlab/blob/master/CHANGELOG.md

## Usage

```rust
// examples

use mathlab::math;

fn main() {
    let abs = math::abs(-2.0);
    let add = math::add(0.1, 0.2);
    let add_f64 = math::add(0.1, 0.2) as f64;

    println!("{}", abs); // 2
    println!("{}", add); // 0.30000000000000004
    println!("{}", add_f64); // 0.30000000000000004
    println!("{}", math::add(0.1, 0.2) as f32); // 0.3
    println!("{}", math::fix64(0.1 + 0.2)); // 0.3
    println!("{}", math::fix(0.1 + 0.2, 15)); // 0.3
    println!("{}", math::to_fixed(0.1 + 0.2, 15)); // "0.3"

    println!(
        "{:?}",
        math::subt_vec_vec(
            &[0.1, 0.2, 0.3], &[0.3, 0.2, 0.1]
        )
    ); // [-0.19999999999999998, 0.0, 0.19999999999999998]

    println!(
        // with vectors, use "{:?}" or "{:#?}".
        "{:?}",
        math::fix64_vec(
            // Use the reference (&) before vector.
            &math::subt_vec_vec(&[0.1, 0.2, 0.3], &[0.3, 0.2, 0.1]
        ))
    ); // [-0.2, 0.0, 0.2]

    println!(
        "{:?}",
        math::sin_vec(&[0.5235987756, 1.5707963268])
    ); // [0.5, 1.0]

    // [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
    println!("{:?}", math::range(0.0, 0.1, 11, "asc"));

    // [1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0]
    println!("{:?}", math::range(1.0, 0.1, 11, "desc"));

    println!(
        "{:#?}",
        math::sin_deg_vec(&math::range(0.0, 1.0, 361, "asc"))
    ); // [0.0, ..., 0.5, ..., 1.0, ..., 0.5, ..., 0.0]
}
```

or

```rust
// examples

use mathlab::math::{
    abs, add, fix64, fix64_vec, fix,
    to_fixed, range, sin_vec,
    sin_deg_vec, subt_vec_vec
};

fn main() {
    let my_abs = abs(-2.0);
    let my_add = add(0.1, 0.2);
    let my_add_f64 = add(0.1, 0.2) as f64;

    println!("{}", my_abs); // 2
    println!("{}", my_add); // 0.30000000000000004
    println!("{}", my_add_f64); // 0.30000000000000004
    println!("{}", add(0.1, 0.2) as f32); // 0.3
    println!("{}", fix64(0.1 + 0.2)); // 0.3
    println!("{}", fix(0.1 + 0.2, 15)); // 0.3
    println!("{}", to_fixed(0.1 + 0.2, 15)); // "0.3"

    println!(
        "{:?}",
        subt_vec_vec(
            &[0.1, 0.2, 0.3], &[0.3, 0.2, 0.1]
        )
    ); // [-0.19999999999999998, 0.0, 0.19999999999999998]

    println!(
        // with vectors, use "{:?}" or "{:#?}".
        "{:?}",
        fix64_vec(
            // Use the reference (&) before vector.
            &subt_vec_vec(&[0.1, 0.2, 0.3], &[0.3, 0.2, 0.1]
        ))
    ); // [-0.2, 0.0, 0.2]

    println!(
        "{:?}",
        sin_deg_vec(&[30.0, 90.0])
    ); // [0.5, 1.0]

    // [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
    println!("{:?}", range(0.0, 0.1, 11, "asc"));

    // [1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0]
    println!("{:?}", range(1.0, 0.1, 11, "desc"));

    println!(
        "{:#?}",
        sin_deg_vec(&range(0.0, 1.0, 361, "asc"))
    ); // [0.0, ..., 0.5, ..., 1.0, ..., 0.5, ..., 0.0]
}
```

## Web 2.0 Demo Project

- **[on Vercel](https://mathlab-yew.vercel.app)**&emsp;[![vercel](https://img.shields.io/badge/vercel-MathLab%20+%20yew-555555?style=for-the-badge&labelColor=000000&logo=vercel)](https://mathlab-yew.vercel.app)

- **[on GitHub](https://github.com/dr-montasir/mathlab-yew)**&nbsp;&nbsp;[![github](https://img.shields.io/badge/github-MathLab%20+%20yew-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dr-montasir/mathlab-yew)

## Web 3.0 Demo Project

### Canister | Smart Contract |

> On the [Internet Computer](https://internetcomputer.org/), <b>smart contracts</b> come in the form of [canisters](https://internetcomputer.org/how-it-works/canister-lifecycle/). These are computational units which bundle together code and state.

- **[on Internet Computer](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=ox3r2-3aaaa-aaaag-allya-cai)**&emsp;[![Internet Computer](https://img.shields.io/badge/Internet%20Computer-MathLab%20+%20ic-8d3590?style=for-the-badge&labelColor=150d5a&logo=logo)](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=ox3r2-3aaaa-aaaag-allya-cai)

- **[on GitHub](https://github.com/dr-montasir/mathlab-ic)**&nbsp;&nbsp;[![github](https://img.shields.io/badge/github-MathLab%20+%20ic-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dr-montasir/mathlab-ic)

## Documentation

> #### **[https://docs.rs/mathlab](https://docs.rs/mathlab)**
>
> **MathLab :**
>
> A Powerful Math Library for Rust
>
> [All Items](https://docs.rs/mathlab/latest/mathlab/all.html)&emsp;<small>[ 158 ]</small>

### Modules

### 1. [constants](https://docs.rs/mathlab/latest/mathlab/constants/index.html)&emsp;<small>[ 16 items ]</small>

|                                     constant                                      |                                    constant                                     |                                     constant                                      |
| :-------------------------------------------------------------------------------: | :-----------------------------------------------------------------------------: | :-------------------------------------------------------------------------------: |
|       [E](https://docs.rs/mathlab/latest/mathlab/constants/constant.E.html)       |   [H_PI](https://docs.rs/mathlab/latest/mathlab/constants/constant.H_PI.html)   | [INF_F32](https://docs.rs/mathlab/latest/mathlab/constants/constant.INF_F32.html) |
| [INF_F64](https://docs.rs/mathlab/latest/mathlab/constants/constant.INF_F64.html) |    [LN2](https://docs.rs/mathlab/latest/mathlab/constants/constant.LN2.html)    |    [LN10](https://docs.rs/mathlab/latest/mathlab/constants/constant.LN10.html)    |
|   [LOG2E](https://docs.rs/mathlab/latest/mathlab/constants/constant.LOG2E.html)   | [LOG10E](https://docs.rs/mathlab/latest/mathlab/constants/constant.LOG10E.html) | [NAN_F32](https://docs.rs/mathlab/latest/mathlab/constants/constant.NAN_F32.html) |

[NAN_F64](https://docs.rs/mathlab/latest/mathlab/constants/constant.NAN_F64.html)&emsp;[NINF_F32](https://docs.rs/mathlab/latest/mathlab/constants/constant.NINF_F32.html)&emsp;[NINF_F64](https://docs.rs/mathlab/latest/mathlab/constants/constant.NINF_F64.html)&emsp;[PHI](https://docs.rs/mathlab/latest/mathlab/constants/constant.PHI.html)&emsp;[PI](https://docs.rs/mathlab/latest/mathlab/constants/constant.PI.html)&emsp;[Q_PI](https://docs.rs/mathlab/latest/mathlab/constants/constant.Q_PI.html)&emsp;[TAU](https://docs.rs/mathlab/latest/mathlab/constants/constant.TAU.html)

### 2. [functions](https://docs.rs/mathlab/latest/mathlab/functions/index.html)&emsp;<small>[ 142 items ]</small>

- ### [args](https://docs.rs/mathlab/latest/mathlab/functions/args/index.html)&emsp;<small>[ 3 items ]</small>

|                                      function                                      |                                   function                                   |                                      function                                      |
| :--------------------------------------------------------------------------------: | :--------------------------------------------------------------------------: | :--------------------------------------------------------------------------------: |
| [monolist](https://docs.rs/mathlab/latest/mathlab/functions/args/fn.monolist.html) | [range](https://docs.rs/mathlab/latest/mathlab/functions/args/fn.range.html) | [to_fixed](https://docs.rs/mathlab/latest/mathlab/functions/args/fn.to_fixed.html) |

- ### [num](https://docs.rs/mathlab/latest/mathlab/functions/num/index.html)&emsp;<small>[ 69 items ]</small>

|                                     function                                      |                                     function                                      |                                     function                                      |
| :-------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------: |
|      [abs](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.abs.html)      |      [add](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.add.html)      |     [acot](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.acot.html)     |
| [acot_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.acot_deg.html) |     [acos](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.acos.html)     | [acos_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.acos_deg.html) |
|     [acsc](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.acsc.html)     | [acsc_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.acsc_deg.html) |     [asec](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.asec.html)     |

[asec_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.asec_deg.html)&emsp;[asin](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.asin.html)&emsp;[asin_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.asin_deg.html)&emsp;[atan](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.atan.html)&emsp;[atan_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.atan_deg.html)&emsp;[cbrt](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.cbrt.html)&emsp;[ceil](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.ceil.html)&emsp;[cot](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.cot.html)&emsp;[cot_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.cot_deg.html)&emsp;[coth](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.coth.html)&emsp;[cos](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.cos.html)&emsp;[cos_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.cos_deg.html)&emsp;[cosh](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.cosh.html)&emsp;[csc](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.csc.html)&emsp;[csc_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.csc_deg.html)&emsp;[csch](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.csch.html)&emsp;[cube](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.cube.html)&emsp;[deg_to_rad](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.deg_to_rad.html)&emsp;[divi](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.divi.html)&emsp;[exp](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.exp.html)&emsp;[f64_to_f32](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.f64_to_f32.html)&emsp;[fact](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.fact.html)&emsp;[fix](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.fix.html)&emsp;[fix64](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.fix64.html)&emsp;[floor](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.floor.html)&emsp;[fround](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.fround.html)&emsp;[gamma](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.gamma.html)&emsp;[i64_to_f64](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.i64_to_f64.html)&emsp;[inv](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.inv.html)&emsp;[is_inf_f32](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.is_inf_f32.html)&emsp;[is_inf_f64](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.is_inf_f64.html)&emsp;[is_nan_f32](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.is_nan_f32.html)&emsp;[is_nan_f64](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.is_nan_f64.html)&emsp;[is_ninf_f32](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.is_ninf_f32.html)&emsp;[is_ninf_f64](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.is_ninf_f64.html)&emsp;[ln](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.ln.html)&emsp;[ln1p](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.ln1p.html)&emsp;[log2](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.log2.html)&emsp;[log10](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.log10.html)&emsp;[mult](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.mult.html)&emsp;[nrt](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.nrt.html)&emsp;[pow](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.pow.html)&emsp;[rad_to_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.rad_to_deg.html)&emsp;[rem](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.rem.html)&emsp;[round](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.round.html)&emsp;[sec](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.sec.html)&emsp;[sec_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.sec_deg.html)&emsp;[sech](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.sech.html)&emsp;[sign](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.sign.html)&emsp;[sin](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.sin.html)&emsp;[sin_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.sin_deg.html)&emsp;[sinh](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.sinh.html)&emsp;[sqr](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.sqr.html)&emsp;[sqrt](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.sqrt.html)&emsp;[subt](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.subt.html)&emsp;[tan](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.tan.html)&emsp;[tan_deg](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.tan_deg.html)&emsp;[tanh](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.tanh.html)&emsp;[trunc](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.trunc.html)&emsp;[u64_to_f64](https://docs.rs/mathlab/latest/mathlab/functions/num/fn.u64_to_f64.html)

- ### [vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/index.html)&emsp;<small>[ 49 items ]</small>

|                           function                           |                           function                           |                           function                           |
| :----------------------------------------------------------: | :----------------------------------------------------------: | :----------------------------------------------------------: |
| [abs_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.abs_vec.html) | [acos_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.acos_vec.html) | [acos_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.acos_deg_vec.html) |
| [acot_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.acot_vec.html) | [acot_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.acot_deg_vec.html) | [acsc_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.acsc_vec.html) |
| [acsc_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.acsc_deg_vec.html) | [asec_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.asec_vec.html) | [asec_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.asec_deg_vec.html) |

[asin_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.asin_vec.html)&emsp;[asin_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.asin_deg_vec.html)&emsp;[atan_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.atan_vec.html)&emsp;[atan_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.atan_deg_vec.html)&emsp;[cbrt_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.cbrt_vec.html)&emsp;[ceil_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.ceil_vec.html)&emsp;[cos_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.cos_vec.html)&emsp;[cos_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.cos_deg_vec.html)&emsp;[cot_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.cot_vec.html)&emsp;[cot_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.cot_deg_vec.html)&emsp;[csc_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.csc_vec.html)&emsp;[csc_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.csc_deg_vec.html)&emsp;[cube_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.cube_vec.html)&emsp;[deg_to_rad_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.deg_to_rad_vec.html)&emsp;[exp_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.exp_vec.html)&emsp;[f64_to_f32_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.f64_to_f32_vec.html)&emsp;[fact_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.fact_vec.html)&emsp;[fix64_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.fix64_vec.html)&emsp;[floor_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.floor_vec.html)&emsp;[fround_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.fround_vec.html)&emsp;[gamma_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.gamma_vec.html)&emsp;[i64_to_f64_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.i64_to_f64_vec.html)&emsp;[inv_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.inv_vec.html)&emsp;[ln_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.ln_vec.html)&emsp;[ln1p_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.ln1p_vec.html)&emsp;[log2_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.log2_vec.html)&emsp;[log10_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.log10_vec.html)&emsp;[rad_to_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.rad_to_deg_vec.html)&emsp;[round_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.round_vec.html)&emsp;[sec_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.sec_vec.html)&emsp;[sec_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.sec_deg_vec.html)&emsp;[sign_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.sign_vec.html)&emsp;[sin_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.sin_deg_vec.html)&emsp;[sin_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.sin_vec.html)&emsp;[sqr_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.sqr_vec.html)&emsp;[sqrt_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.sqrt_vec.html)&emsp;[tan_deg_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.tan_deg_vec.html)&emsp;[tan_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.tan_vec.html)&emsp;[trunc_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.trunc_vec.html)&emsp;[u64_to_f64_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec/fn.u64_to_f64_vec.html)

- ### [num_vec](https://docs.rs/mathlab/latest/mathlab/functions/num_vec/index.html)&emsp;<small>[ 7 items ]</small>

|                                           function                                            |                                           function                                            |                                           function                                            |
| :-------------------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------: |
|  [add_num_vec](https://docs.rs/mathlab/latest/mathlab/functions/num_vec/fn.add_num_vec.html)  | [divi_num_vec](https://docs.rs/mathlab/latest/mathlab/functions/num_vec/fn.divi_num_vec.html) | [mult_num_vec](https://docs.rs/mathlab/latest/mathlab/functions/num_vec/fn.mult_num_vec.html) |
|  [nrt_num_vec](https://docs.rs/mathlab/latest/mathlab/functions/num_vec/fn.nrt_num_vec.html)  |  [pow_num_vec](https://docs.rs/mathlab/latest/mathlab/functions/num_vec/fn.pow_num_vec.html)  |  [rem_num_vec](https://docs.rs/mathlab/latest/mathlab/functions/num_vec/fn.rem_num_vec.html)  |
| [subt_num_vec](https://docs.rs/mathlab/latest/mathlab/functions/num_vec/fn.subt_num_vec.html) |                                                                                               |                                                                                               |

- ### [vec_num](https://docs.rs/mathlab/latest/mathlab/functions/vec_num/index.html)&emsp;<small>[ 7 items ]</small>

|                                           function                                            |                                           function                                            |                                           function                                            |
| :-------------------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------: |
|  [add_vec_num](https://docs.rs/mathlab/latest/mathlab/functions/vec_num/fn.add_vec_num.html)  | [divi_vec_num](https://docs.rs/mathlab/latest/mathlab/functions/vec_num/fn.divi_vec_num.html) | [mult_vec_num](https://docs.rs/mathlab/latest/mathlab/functions/vec_num/fn.mult_vec_num.html) |
|  [nrt_vec_num](https://docs.rs/mathlab/latest/mathlab/functions/vec_num/fn.nrt_vec_num.html)  |  [pow_vec_num](https://docs.rs/mathlab/latest/mathlab/functions/vec_num/fn.pow_vec_num.html)  |  [rem_vec_num](https://docs.rs/mathlab/latest/mathlab/functions/vec_num/fn.rem_vec_num.html)  |
| [subt_vec_num](https://docs.rs/mathlab/latest/mathlab/functions/vec_num/fn.subt_vec_num.html) |                                                                                               |                                                                                               |

- ### [vec_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec_vec/index.html)&emsp;<small>[ 7 items ]</small>

|                                           function                                            |                                           function                                            |                                           function                                            |
| :-------------------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------: |
|  [add_vec_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec_vec/fn.add_vec_vec.html)  | [divi_vec_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec_vec/fn.divi_vec_vec.html) | [mult_vec_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec_vec/fn.mult_vec_vec.html) |
|  [nrt_vec_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec_vec/fn.nrt_vec_vec.html)  |  [pow_vec_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec_vec/fn.pow_vec_vec.html)  |  [rem_vec_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec_vec/fn.rem_vec_vec.html)  |
| [subt_vec_vec](https://docs.rs/mathlab/latest/mathlab/functions/vec_vec/fn.subt_vec_vec.html) |                                                                                               |                                                                                               |

### 3. [math](https://docs.rs/mathlab/latest/mathlab/math/index.html)&emsp;<small>[ 158 items ]</small>

The math module contains all constants and functions.

### [All Items](https://docs.rs/mathlab/latest/mathlab/all.html)&emsp;<small>[ 158 ]</small>

[![crates.io](https://img.shields.io/crates/v/mathlab.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/mathlab)

[![docs.rs](https://img.shields.io/badge/docs.rs-mathlab-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/mathlab)

[![license](https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache)](https://choosealicense.com/licenses/apache-2.0)

[![github](https://img.shields.io/badge/github-dr%20montasir%20/%20mathlab-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dr-montasir/mathlab)
