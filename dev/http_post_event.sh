curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "header": { "powerrouter_id":"NOT0THE0REAL0ONE", "verification":0, "time_send":"2014-04-05T10:42:49+01:00", "ver":1}, "event":"A85BC60B0C00100001000200000000000000000000000100000050001A90760000009001B9D30000C8000000" }' \
  http://localhost:8000/events.json
