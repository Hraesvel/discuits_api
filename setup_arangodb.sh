#!/usr/bin/env bash

JS_FILE="setup_arangodb_test_env.js"
if command -v arangosh &> /dev/null
then
  if test -f $JS_FILE   &> /dev/null
    then
      echo "hello"
  #      arangosh --javascript-execute setup_arangodb_test_env.js
  else
      echo $"Can not file \'$JS_FILE\' in current directory"
  fi
else
      echo "Arangodb is required for this program"
fi
