# Guild Wars 2 Boss Time Estimator

Although I have no idea why I've done this. The estimator is used to estimate the time squad need to end a phase to a boss.

There's three categories of dps: power, semi-burst and condi. Power is any normal power dps; semi-burst is condi build which has some burst, basically weaver and firebrand; condi is other condi builds that do not burst.

The file format is `.csv`, since I'm lazy to support other formats.

For DPS that is in `dps/` directory, the format is `Time,DPS`. You must fill in all 3 types of dps to get the result.

For encounters that is in `data/` directory, the format is `Phase,HP,Coeff,PCoeff,DPSCount`. Phase is the index of phase, start from 1; HP is the total health of the phase; Coeff is the mechanic coefficient of the boss; PCoeff is the coefficient only applied on power builds, which is the lower toughness; DPSCount is the ratio of squad dps to personal dps.
