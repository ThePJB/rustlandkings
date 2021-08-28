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

gun bursting kind of fixed
what next
enemy ai? random wander
behaviour trees??? 
better level generation?

behav trees hectic i think for this maybe just code it...
just pick state vs
fsm vs
behav trees

just want em to wander, stand, pursue, engage, dodge, flee
idle vs in combat
dont accidently make a programming language lol

--------------------
entities need a LookDir so guns can keep shootin
    ok done now i can make guns keep shooting somehow. through AI or updating guns? updating guns seems better
    
and also for their looking
grid needs a raycast for visibility

------------
honestly think typed normals are a good idea
bruh my early returns in rust so hard. oh nah ? operator for that. sick as

-----------

ok so its marching the ray in the wrong direction and incrementing x by infinity

ok it kind of works it sometimes gets stuck in infinite loops.
might be broken in the - direction case

----------
wait wtf at the one where it should return Some(pos) it actually stalls for a turn and then misses it
also suspicious amount of y movement in this test
next tile in doesnt make any goddamn sense

nearly perf
there is a deadzone tho
e in middle p in bottom right of square
####
##p#
#e #
####