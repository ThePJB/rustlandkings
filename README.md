# rustland kings aka squareforce one
## Build instructions
`cargo run` should be sufficient
might need sdl2 or something

## How to play
wasd and click
r to reset



### todos

do the freecell thing of seed leaderboard seederboard. could have like elo
really in 2021 its trivialy easy to get like big web interactive shit running, like that chat room that was on hn. have that in game lobby
that but click to change colour / bump to keep alive, or u can reply and get nodes linked

Gunplay - different gun pickups with different properties

fixing collisions when the parties run at each other: n^2 through collision list
maybe can combine or modify the events to the speed * dt of where they would have ended up

enemies - AI
    - swarmer enemy
    - naviagation system
    - spawn distribution
    - random wander with raycast

terrain generation
    noise?
    graph substitution grammar?
    CA?
    random walk? ---- this one lol. random walkers have a chance to change direction or not. could have each one turn into an entity.

fix sliding along walls.  if you are travelling diagonally you still hitch a bit

design
------

ok its a bit like super crate box

your inventory is just a list of guns and you go through them in order, single use
you can also eat them to regain hp
maybe throw them and they explode or something
alt fire

proc gen guns
effect when eaten etc

maybe take them to somewhere in the world and it juices them but then you have to pick them up and they go to back of queue
and you would be big anticipate

so yeah there would be a lot of snap planning, adapting etc

---------
kek right now its a puzzle game where you kill by using them against each other
ultimate pacifism


------------

command pattern going to be good for sounds and shit


---- 

Generation
----------
Yeah idk maybe author some maps could be cool

Mechanics
---------
Add gun fifo
add eating guns
add hp


Enemies
-------
Sprayer
    sprayer smg cunt tries to get within his short range to shoot at you

Bigdog
    big and slow and tanky, double shotgun