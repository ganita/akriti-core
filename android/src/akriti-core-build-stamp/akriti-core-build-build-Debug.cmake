

set(command "cargo;build;--target;armv7-linux-androideabi")
execute_process(
  COMMAND ${command}
  RESULT_VARIABLE result
  OUTPUT_FILE "/Users/sreejithkrishnanr/Shakuntala/akriti-core/android/src/akriti-core-build-stamp/akriti-core-build-build-out.log"
  ERROR_FILE "/Users/sreejithkrishnanr/Shakuntala/akriti-core/android/src/akriti-core-build-stamp/akriti-core-build-build-err.log"
  )
if(result)
  set(msg "Command failed: ${result}\n")
  foreach(arg IN LISTS command)
    set(msg "${msg} '${arg}'")
  endforeach()
  set(msg "${msg}\nSee also\n  /Users/sreejithkrishnanr/Shakuntala/akriti-core/android/src/akriti-core-build-stamp/akriti-core-build-build-*.log")
  message(FATAL_ERROR "${msg}")
else()
  set(msg "akriti-core-build build command succeeded.  See also /Users/sreejithkrishnanr/Shakuntala/akriti-core/android/src/akriti-core-build-stamp/akriti-core-build-build-*.log")
  message(STATUS "${msg}")
endif()
