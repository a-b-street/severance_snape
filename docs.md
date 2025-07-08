# Severance Snape

## Origins

In Taipei, I delighted in the narrow alleys, which felt 10 degrees cooler than the massive heat island roads. They almost felt like a natural quietway network, except I'd keep hitting a main road, where I'd have to backtrack a few minutes left or right to a crossing, where I'd then wait up to two minutes. At one four-way intersection where I wanted to make a diagonal, I think I remember waiting 5 minutes for crossing each direction. I remember trying to cros the street just north of the main station, walking east five minutes, and then watching people struggle with luggage up a huge pedestrian overpass. (I probably missed the passage through the station.)

In London I cross the street easily in most places, but there are other perplexing severances. At a park or public garden, I might circle around for a minute or two just to find the entrance through the fence. (Where I grew up, public greenspaces don't funnel people through access points this way.) And I'm lucky where I live; some areas are much harder to cross. There is one desire line with no crossing right outside of Elephant Park that I watch people dart across, dodging buses, every day.

While visiting Hong Kong at the end of 2023, I finally started this tool, because I was temporarily hobbling around on crutches, and the pedestrian severances were absurd. It was a simple start. I hope the name is obvious.

...

But more generally, I wanted to finally dedicate a tool to walking as a mode. Cycling is hard enough, but walking is even tougher (due to reasons I'll explain) and there seems to be less focus on it generally.

## Goals

What should a walking tool do?

- Show individual routes, if only just to debug or understand the rest of thetool
- Have isochrones, to feed into a more accurate connectivity analysis
- Emphasize/summarize severances in a simple way (how far between crossings?)
- Model how new pedestrian infrastructure could improve things
- See how new developments could link into the existing network
- Help prioritize areas for improvement

And all of this with an appropriate level of detail:

- Crossings
- Differing walking speeds
- Adjusting speed and reachability for steep streets / stairs
- 

- Penalizing narrow pavements next to busy roads
- 


    - delays at tsigs
      - crossing islands unmarked help
      - this is REALLY place dependent, taipei vs london. the one by mansion house is unusually slow.
      - zebras mean less in US than UK
      - crossing nodes are only shown on severances. weird?
      - need to cluster them... sometimes.
    - limits with this model... all sevs not equal. lanes, frequency of tsigs
      - nobody waits for the signal in london. walworth road and southwark bridge are not equal.
    - want properties per sidewalk... LTS based on traffic speeds, width (need to be bigger in some places), greenery, shops, shade/heat
