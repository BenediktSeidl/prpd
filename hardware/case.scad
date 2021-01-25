pcb_width = 22;
pcb_length = 54.5;
pcb_height = 1.6;
pcb_distance_bottom = 3;

plug_width = 16.41;
plug_length = 15.71;
plug_height = 13.11;
plug_excess = 3;

wall_thick = 3;
wall_thin = 2;

case_side();
mirror([0, 1, 0])
case_side();

module case_side() {

ring_depth = 10;

translate([0, pcb_length/2+plug_excess - ring_depth, plug_height])
cube([plug_width/2, ring_depth, wall_thick]);

translate([plug_width/2, pcb_length/2+plug_excess - ring_depth, 0])
cube([wall_thick, ring_depth, wall_thick+plug_height]);

translate([0, pcb_length/2, -pcb_height-pcb_distance_bottom-wall_thin])
cube([pcb_width/2 + wall_thin, plug_excess, pcb_height + pcb_distance_bottom+wall_thin]);

translate([pcb_width/2, 0, -pcb_height]) {
    cube([wall_thin, pcb_length/2, pcb_height]);
    translate([-2, 0, 0]){
        translate([0, 0, pcb_height])
          cube([2+wall_thin,pcb_length/2, wall_thick]);
        translate([0, 0, -pcb_distance_bottom-wall_thin])
        cube([2+wall_thin,pcb_length/2, pcb_distance_bottom+wall_thin]);
    }
}

// thin wall bottom
translate([0, 0, -pcb_height-pcb_distance_bottom-wall_thin])
cube([pcb_width/2, pcb_length/2, wall_thin]);

// thin wall side
color([1, 0, 0])
translate([pcb_width/2, 0, 0])
cube([wall_thin, pcb_length/2 + plug_excess, plug_height + wall_thick]);

}

color([0, 1, 0]) {
 //  pcb();
}

module pcb () {
    translate([-pcb_width/2, -pcb_length/2, -pcb_height]) {
    cube([pcb_width, pcb_length, pcb_height]);
    }

    plug();
    mirror([0, 1, 0]) {
        plug();
    }

    module plug () {
        translate([-(plug_width)*0.5, pcb_length*0.5-plug_length+plug_excess, 0]) {
            cube([plug_width, plug_length, plug_height]);
        }
    }
}