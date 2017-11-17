#!/bin/sh

##
# Copyright 2017 Sreejith Krishnan R
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
##

cd ../

source $HOME/.bash_profile

LIBRARY_NAME=libakriti_ios.a

build_config=$1
arch=$2
product_build_dir=$3

case $build_config in
	Debug)
		release_flag=""
		build_dir="debug"
		;;
    Release)
        release_flag="--release"
        build_dir="release"
        ;;
	*)
		echo "Unrecognized build config $build_config"
		exit
		;;
esac

case $arch in
	x86_64)
		target="x86_64-apple-ios"
		;;
	*)
		echo "Unrecognized arch $arch"
		exit
		;;
esac

cargo lipo $release_flag --targets $target

cp target/universal/$build_dir/$LIBRARY_NAME $product_build_dir/$LIBRARY_NAME
