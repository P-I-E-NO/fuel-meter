# fuel-meter
Very simple microservice that triggers the notification to the user's device.<br>
When the bridge detects that the fuel level is low, it will simply `POST` this endpoint with the current value, longitude and latitude. `POST`ing the endpoint will place an event inside the corresponding Redis stream.
