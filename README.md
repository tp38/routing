# Routing

The main goal of this project is to implement Dijktra and some distance calculation algorythms. This must be done in Rust language.    
We use as input graph data from OpenStreetMap (file with osm.pbf extension).

## Installation

Just download this project and use cargo to build **route**. 

> $ cargo build

## Configuration

By default, data directory contains only one file :    
- St_Brieuc-Loudéac.osm.pbf

Just download the osm.pdf of your country and put it in data directory. 

## Command line args

### Getting help

Just type the below command line :    
> $ cargo run -- -h

```
th@6po:~/Code/Rust/route$ cargo run -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/route -h`
Thierry P. <thierry.probst@free.fr>
 
Some things for routing
 
Version: 1.1.0 
 Usage: route [OPTIONS] 

 Options:
  -f, --filename <FILENAME>  Optional file name to operate on. default is "St_Brieuc-Loudéac"
  -i, --itype <ITYPE>        Optional input file type in ["osm", "osm.pbf"]. default is "osm.pbf"
  -h, --help                 Print help
  -V, --version              Print version
th@6po:~/Code/Rust/route$
```

### Getting version 

Type a bash command line as below :   
> $ cargo run -- -V

```
  th@6po:~/Code/Rust/route$ cargo run -- -V
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/route -V`
  route 1.1.0
  th@6po:~/Code/Rust/route$
```

### Run with a specific map

1. Put your map (osm.pbf file) in data directory
2. Run cargo as follow :
  > $ cargo run -- -f Bretagne

```
th@6po:~/Code/Rust/route$ cargo run -- -f Bretagne
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/route -f Bretagne`
  > info
  file : /mnt/vg1-data/Code/Rust/route/data/Bretagne.osm.pbf
          tways : 55976 , tnodes : 533411
  graph : 
          tways are :
          trunk_link           =>     405
          residential          =>   15507
          tertiary             =>    5075
          trunk                =>     682
          primary              =>    1713
          secondary            =>    2578
          tertiary_link        =>      66
          primary_link         =>      76
          service              =>   14593
          secondary_link       =>      52
          unclassified         =>   15229

  > quit
  th@6po:~/Code/Rust/route$
```

On this example, we also see the `info` (show differents ways count present in graph) and `quit` (same as exit) menu commands.  

## Menu commands

### info

see example in "Run with a specific map". The command show all differents routing ways presents in the graph (osm.pbf file) and the associate number.

### show nodes

display 5 randomly chosen nodes with their coordinates and ways.  

```
  > show nodes
  59996888
          coords : (48.61298540000001 , -1.9824291) :
          ways : 320036627

  1178825217
          coords : (48.333485 , -2.3467975) :
          ways : 102136763 102136768

  7953021513
          coords : (48.5044388 , -2.7630635000000003) :
          tags : 
                  highway => crossing
                  crossing => uncontrolled
                  crossing_ref => zebra
          ways : 425581140

  2205076193
          coords : (48.5561699 , -3.1150281000000004) :
          ways : 210444075

  10042265119
          coords : (48.251452400000005 , -2.3212199) :
          ways : 103406410

  > 
```

### show node node_number

show data about the specified node :   

```
  > show node 10042265119
  10042265119
          coords : (48.251452400000005 , -2.3212199) :
          ways : 103406410

  > 
```

### show ways

show 5 ramdomly chosen ways with all characteristics :   

```
  > show ways
  667691318
          nodes : 6251744733, 6251744734,
          longueur : 62.84363890660165
          tags :
                  highway => service
                  service => parking_aisle
                  surface => asphalt

  270394962
          nodes : 2754163307, 10541847535, 10541847525, 10541721965, 10541847533, 3469261267, 10541721966, 10541721960, 10541847515, 10541847516, 10541721988, 3469261268,
          longueur : 353.71154949577306
          tags :
                  cycleway:left => opposite
                  name => Rue du Tertre de la Motte
                  maxspeed => 50
                  cycleway:right => track
                  highway => unclassified

  116505907
          nodes : 1313208781, 1313208707, 1313208767, 1313208646, 1313208704, 1313208689, 1313208674, 1313208680, 1313208724, 1313208779, 1313208666, 1313208789, 1313208675,
          longueur : 371.37484658465377
          tags :
                  source => cadastre-dgi-fr source : Direction Générale des Impôts - Cadastre. Mise à jour : 2011
                  ref => C 21
                  highway => unclassified

  857535435
          nodes : 10054401255, 7995068719, 7995068720,
          longueur : 97.21927801383212
          tags :
                  highway => service
                  service => parking_aisle
                  surface => asphalt

  128363917
          nodes : 1418802969, 2549610808, 2549610810, 2549610809, 2549610812, 1423434104, 1423434099, 2549610816, 1423434098, 4598933301, 1423434097, 1423434095, 6285692188, 4598933302, 6285692189, 6285692190, 4598933303, 1175713771,
          longueur : 395.27760840245134
          tags :
                  oneway => yes
                  source:name:br => proper translation
                  maxspeed => 50
                  surface => asphalt
                  highway => residential
                  name => Rue du Calvaire
                  name:br => Straed ar C'halvar

  > 
```

### show way way_number

show all data known for specific way designed by is number :    

```
  > show way 857535435
  857535435
          nodes : 10054401255, 7995068719, 7995068720,
          longueur : 97.21927801383212
          tags :
                  highway => service
                  service => parking_aisle
                  surface => asphalt

  >
```

### locate address

give ways corresponding to an address.  

```
  > locate 1 bis rue de broceliande 22000 St Brieuc

  1 bis rue de broceliande 22000 St Brieuc :
  189440481
          nodes : 2000599143, 7856977869, 7152330125, 2000599145, 2000599147, 8120585516, 2000599152, 2000599154, 8120585515, 2000599135, 8120585514, 2000599141, 1825134089,
          longueur : 233.3182283305317
          tags :
                  oneway => no
                  highway => residential
                  lanes => 2
                  name => Rue de Brocéliande
                  old_name => Chemin Vicinal Ordinaire n°15
                  name:br => Straed Breselien
                  name:etymology:wikidata => Q2925919
                  source => cadastre-dgi-fr source : Direction Générale des Impôts - Cadastre. Mise à jour : 2020;BDOrtho IGN - 2020
                  source:name:br => proper translation
                  start_date => before 1959
                  name:start_date => 1959
                  surface => asphalt
                  maxspeed => 30
                  lit => yes

  ....

  96364872
          nodes : 294177926, 4269660995, 1116770288, 1116770367, 2166715632, 1116770539,
          longueur : 177.3374925859465
          tags :
                  name => Rue de Brocéliande
                  name:br => Straed Breselien
                  source:name:br => proper translation
                  highway => residential

  > 
```

### nearest lat lon

Give the nearest node to the given coordinates (atitude and longitude)   

```
  > nearest 48.44725 -2.86572
  le point 2345943396 est le plus proche à 17.28 m
  > 
```

### route distance node_1 node_2

Give the shortest path (calcul with the distance) between two nodes    

```
> route distance 10748130358 4779385124
  0 m : 
  10748130358
          coords : (48.4063898 , -2.8150775) :
          ways : 96378518 132958754 1155758352

          96378518 : Rue du Beau Chemin
          132958754 : Rue de la Clé des Champs
          1155758352 : Rue du Beau Chemin
  40.63 m : 
  294179001
          coords : (48.406753800000004 , -2.8151267) :
          ways : 318688404 1155758352

          318688404 : Rue Notre-Dame
          1155758352 : Rue du Beau Chemin
  51.82 m : 
  10312984908
          coords : (48.406758 , -2.8149753) :
          tags : 
                  highway => crossing
                  crossing => uncontrolled
          ways : 318688404

          318688404 : Rue Notre-Dame
  54.36 m : 
  4779385123
          coords : (48.406758100000005 , -2.8149408) :
          ways : 318688404

          318688404 : Rue Notre-Dame
  82.36 m : 
  4779385124
          coords : (48.4067937 , -2.8145653000000004) :
          ways : 318688404

          318688404 : Rue Notre-Dame
  > 
```
82.86 m means 82.86 meters long :wink:.

### route time node_1 node_2

Same as `route distance` but use time to search the shortest path.

```
> route time 10748130358 4779385124
  0 m : 
  10748130358
          coords : (48.4063898 , -2.8150775) :
          ways : 96378518 132958754 1155758352

          96378518 : Rue du Beau Chemin
          132958754 : Rue de la Clé des Champs
          1155758352 : Rue du Beau Chemin
  0.34 m : 
  294179001
          coords : (48.406753800000004 , -2.8151267) :
          ways : 318688404 1155758352

          318688404 : Rue Notre-Dame
          1155758352 : Rue du Beau Chemin
  1.58 m : 
  10312984908
          coords : (48.406758 , -2.8149753) :
          tags : 
                  crossing => uncontrolled
                  highway => crossing
          ways : 318688404

          318688404 : Rue Notre-Dame
  7.03 m : 
  4779385123
          coords : (48.406758100000005 , -2.8149408) :
          ways : 318688404

          318688404 : Rue Notre-Dame
  7.53 m : 
  4779385124
          coords : (48.4067937 , -2.8145653000000004) :
          ways : 318688404

          318688404 : Rue Notre-Dame
  >
```

7.53 m means 7.53 minutes long :wink:.

### gpx mode node_1 node_2

mode is in [ "distance", "time" ]
Same as `route mode node_1 node_2` (see above paragraphs) but save result in data\trace.gpx file. 
  
This file can be read by a viewer like **GPX viewer**
