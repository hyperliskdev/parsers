# parsers
COMP625 - Network Forensics | Lab 5 Parsers


### Lab 5 - Parsers

In this lab, I will be creating a parsing service for an older firewall written by a retired programmer at an organization. The firewall is called vernbox, created by Vern.

A sample log has been provided (vernlog.csv) and the goal of this program is to take http requests from the vernbox and return http requests structured as json instead of csv.

The returned request should look something like this -->

```
{
  "clientaddr": "62.42.91.6",
  "destaddr": "192.160.80.91",
  "firewall": "vern1",
  "port": "80",
  "proto": "http"
  "action" "allow"


  // Added Field \\
  "country": "GB"

}
```

In addition to this data that is already in the vernbox firewall, we should add a country tag that includes just the country label (two letters).


