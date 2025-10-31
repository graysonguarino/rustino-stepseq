# Things on this breadboard which don't match schematic

* Instead of using the 78L05 voltage regulator, I am simply plugging a 9v battery
directly into the `VIN` pin of the Arduino (which can handle 7-12v) and taking the
voltage from the Arduino 5V pin as my positive power rail.
* One of the potentiometers is a 250k pot instead of a 100k pot, but this is only
because I've run out of 100ks.
* TRRS 1/8" jacks are used instead of TS as I had these lying around. It doesn't make
a difference as the two rings are not connected to anything.
* I am using independent buttons instead of a 3-way switch for `RST/ZERO` and `FWD/BWD`
because the switches I have don't fit the breadboard and I don't yet have a way to get
them onto the board without soldering (which I'm trying to avoid for a temporary thing).
