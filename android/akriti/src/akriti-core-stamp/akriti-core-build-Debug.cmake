

set(command "cargo;build;--target;aarch64-linux-android")
execute_process(
  COMMAND ${command}
  RESULT_VARIABLE result
  OUTPUT_FILE "/Users/sreejithkrishnanr/Shakuntala/akriti-core/android/akriti/src/akriti-core-stamp/akriti-core-build-out.log"
  ERROR_FILE "/Users/sreejithkrishnanr/Shakuntala/akriti-core/android/akriti/src/akriti-core-stamp/akriti-core-build-err.log"
  )
if(result)
  set(msg "Command failed: ${result}\n")
  foreach(arg IN LISTS command)
    set(msg "${msg} '${arg}'")
  endforeach()
  set(msg "${msg}\nSee also\n  /Users/sreejithkrishnanr/Shakuntala/akriti-core/android/akriti/src/akriti-core-stamp/akriti-core-build-*.log")
  message(FATAL_ERROR "${msg}")
else()
  set(msg "akriti-core build command succeeded.  See also /Users/sreejithkrishnanr/Shakuntala/akriti-core/android/akriti/src/akriti-core-stamp/akriti-core-build-*.log")
  message(STATUS "${msg}")
endif()
