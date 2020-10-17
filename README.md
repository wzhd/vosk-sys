## Building

### Kaldi

First install dependencies and build the speech recognition toolkit Kaldi,
which Vosk is based on.

Get the code, which contains instructions and some scripts:

> git clone -b lookahead --single-branch https://github.com/alphacep/kaldi

Head over to the `tools` directory: `cd kaldi/tools`,
and prepare some dependencies.
 
Running `make` may just work.
Read the text file `INSTALL` for more info.

#### Linear Algebra Library

A library that provides BLAS and LAPACK is needed to perform
[matrix operations](http://kaldi-asr.org/doc/matrixwrap.html).

##### a) ATLAS
ATLAS (Automatically Tuned Linear Algebra Software)
provides adequate support of BLAS and LAPACK for Kaldi.
It may be available as precompiled packages.
For example on Debian:

> apt install libatlas-base-dev

##### b) OpenBLAS

Alternative implementation of BLAS and parts of LAPACK.
Run script `extras/install_openblas.sh` to download and compile.

##### c) Intel-mkl

Yet another option.
Script: `extras/install_mkl.sh`.

#### Kaldi compilation

Go to `kaldi/src`: `cd ../src`

Configure with `mathlib` set to the linear algebra library used:

`./configure --mathlib=ATLAS --shared --use-cuda=no`

Compile. Various object archive files should be available afterwards (such as `nnet3/kaldi-nnet3.a`):

`make -j 4`

### vosk-sys

`cargo` needs to find Kaldi.
Set the environment variable `KALDI_ROOT` to the path of the Kaldi repository.

C++ code of vosk is added as a submodule.
Run `git submodule init` and `git submodule update`
to get the code.

Now `cargo build` can be run.

