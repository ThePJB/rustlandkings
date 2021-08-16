rustland kings

how to best do collision between terrain and dudes?

currently its like

list of entities
simulate collisions
compute movement
apply movement

so we could have entities -> terrain and rejoin at compute movement step

we could join earlier by just having maybe object
since it just uses the objects rect anyway

---------
compute entity collisions with terrain
    do a neighbourhood search around the entity then
    if entity size < terrain grid size we can just look up the 3x3 area around the entity
    then make collision events like the other function
    then it should be taken care of in compute movement sort of thing

fix terrain stitching

shooting

barrels

enemies


terrain generation
    noise?
    graph substitution grammar?
    CA?
    random walk?

fix sliding along walls

{
    THING: 1.0
    OTHER_THING: 2.0
    asddfasdf: 4.12
}[choice]


------------------

ways in which terrian collision is broken:
    - sometimes ur 1 away from a wall and it thinks u collide with it e.g. 1,0 when ur in 1,1

    rarely as fuck does collision with the actual wall register

    grid rects should be correct because it draws properly, but its as if they get moved and moved offscreen eventually

    does it depend on grid w and h?
    yes it does hmm

    grid picking etc should be correct