# HTTP Communication

*Sensitive information are redacted with `XXX`*

## HTTP Request

```
POST /logs.json HTTP/1.1
Connection: close
Accept: */*
Content-Type: application/json
Host: logging1.powerrouter.com
Content-Length: 1109

{"header":{"powerrouter_id":"9XXXXXXXXXXXXXX5","time_send":"2021-01-20T21:43:01+01:00","version":3,"period":60},"module_statuses":[{"module_id":16,"status":51475,"version":1,"param_0":5004,"param_1":2273,"param_2":180,"param_3":1,"param_4":13729704,"param_5":4833663},{"module_id":9,"status":16147,"version":1,"param_0":5004,"param_1":2283,"param_2":-164,"param_3":4212565,"param_4":6999485,"param_5":2259,"param_6":0,"param_7":1368,"param_8":38566,"param_9":-167,"param_10":308},{"module_id":136,"status":16707,"version":1,"param_0":5225,"param_1":463,"param_2":249,"param_3":5466602,"param_4":5781047,"param_5":59,"param_6":765,"param_7":190,"param_8":438,"param_9":5810,"param_10":7500,"param_11":4523,"param_12":11000},{"module_id":11,"status":1299,"version":1,"param_0":2286,"param_1":82,"param_2":-159,"param_3":7749700,"param_4":2286,"param_5":55,"param_6":121,"param_7":4791900,"param_8":2268,"param_9":26,"param_10":39,"param_11":4177800},{"module_id":14,"status":1043,"version":1,"param_0":0,"param_1":0,"param_2":0,"param_3":-3,"param_4":9,"param_5":0,"param_6":0,"param_7":15,"param_8":24832800}]}
```

Payload formated:

``` json
{
  "header": {
    "powerrouter_id": "9XXXXXXXXXXXXXX5",
    "time_send": "2021-01-20T21:43:01+01:00",
    "version": 3,
    "period": 60
  },
  "module_statuses": [
    {
      "module_id": 16,
      "status": 51475,
      "version": 1,
      "param_0": 5004,
      "param_1": 2273,
      "param_2": 180,
      "param_3": 1,
      "param_4": 13729704,
      "param_5": 4833663
    },
    {
      "module_id": 9,
      "status": 16147,
      "version": 1,
      "param_0": 5004,
      "param_1": 2283,
      "param_2": -164,
      "param_3": 4212565,
      "param_4": 6999485,
      "param_5": 2259,
      "param_6": 0,
      "param_7": 1368,
      "param_8": 38566,
      "param_9": -167,
      "param_10": 308
    },
    {
      "module_id": 136,
      "status": 16707,
      "version": 1,
      "param_0": 5225,
      "param_1": 463,
      "param_2": 249,
      "param_3": 5466602,
      "param_4": 5781047,
      "param_5": 59,
      "param_6": 765,
      "param_7": 190,
      "param_8": 438,
      "param_9": 5810,
      "param_10": 7500,
      "param_11": 4523,
      "param_12": 11000
    },
    {
      "module_id": 11,
      "status": 1299,
      "version": 1,
      "param_0": 2286,
      "param_1": 82,
      "param_2": -159,
      "param_3": 7749700,
      "param_4": 2286,
      "param_5": 55,
      "param_6": 121,
      "param_7": 4791900,
      "param_8": 2268,
      "param_9": 26,
      "param_10": 39,
      "param_11": 4177800
    },
    {
      "module_id": 14,
      "status": 1043,
      "version": 1,
      "param_0": 0,
      "param_1": 0,
      "param_2": 0,
      "param_3": -3,
      "param_4": 9,
      "param_5": 0,
      "param_6": 0,
      "param_7": 15,
      "param_8": 24832800
    }
  ]
}
```

## HTTP Response

```
HTTP/1.1 201 Created
Server: nginx
Date: Wed, 20 Jan 2021 20:43:50 GMT
Content-Type: application/json; charset=utf-8
Content-Length: 34
Connection: close
Status: 201 Created
Cache-Control: max-age=0, private, must-revalidate
ETag: W/"bXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX3"
X-Frame-Options: SAMEORIGIN
X-XSS-Protection: 1; mode=block
X-Content-Type-Options: nosniff
X-Request-Id: 8XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX4

{"next-log-level":2,"status":"ok"}
```

## Data interpretation

There was no own research; Names units and factors where obtained from the
[p-rout](https://github.com/trebb/p-rout) project.

## Theory of operation

As the online service was shut down on 2021-02-01 there is no reason to send
the data to the upstream service (as done by the p-rout project). So all we
have to do is control the DNS-Server that the PowerRouter speaks to, and give
it a IP-Address of the local Network where the prpd HTTP-Server runs.

The PowerRouter sends data every minute.
