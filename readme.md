### Chargepoint Lot Power Demand Simulator

#### _Answer to bonus questions_:

##### Question 1 (_Run the program from task 1 for between 1 and 30 chargepoints. How does the concurrency factor behave?_):
Concurrency factor will decrease as number of chargepoints increase if demand stays level, but the decrease in the concurrency factor will level off as you add more and more chargepoints. The inverse will also occur, wherein reducing the number of chargepoints increases the likelihood that all chargepoints will be occupied at once creating a concurrency factor of 1. 

##### Question 2 (_If you consider the impact of DST vs mapping the hours to the 15 minute ticks?_):
DST won't impact metrics like the concurrency factor, yearly power draw, etc. You may see a slight difference in useage per tick as people may be slightly early/late in their routines when adapting to the time change. Leap years won't effect concurrency factor / simulated demand but should increase yearly power consumption by something like 1/365th.  

##### Question 3 (_If you seed the probabilities vs using random() for random-but-deterministic results?_): 
With the seeded probabilities provided for the task you've outlined a pattern that likely follows real usage patterns - charge demand increases a lot at rush hours, and is increased somewhat by people being both awake and off work. If you use RNG to determine the likelihood of vehicles arriving at a chargepoint and their charge needs over a large number of chargepoints that pattern should average out into more consistent demand.