/*
 * Copyright 2017 Sreejith Krishnan R
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

#include "akriti-helloworld.h"

#include <jni.h>
#include <string>

extern "C"
JNIEXPORT jstring JNICALL
Java_io_ganita_android_akriti_HelloWorld_stringFromJNI(JNIEnv *env, jobject instance, jstring _name) {
    auto name = env->GetStringUTFChars(_name, NULL);
    auto greeting =  env->NewStringUTF(akriti_helloworld(name));
    env->ReleaseStringUTFChars(_name, name);

    return greeting;
}
