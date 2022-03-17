//use std::convert::TryInto;

fn main() {
    /*
metodo simplex

una fabrica produce televisores y computadoras necesitan 
pasar por las lineas de montaje y acabado que disponen con 120 y 180 horas.
las computadoras estan 3 horas en ambas secciones. Los televisores 
necesitan 3hs en montaje y 6 hs en acabado.
la fabrica obtiene un beneficio de $300 por cada computadora y $400 por cada televisor
¿Que cantidad de cada producto debe producir la fabrica para maximizar sus ganancias?

definir datos			
Productos	    Montaje	Acabado	Beneficio
Computadoras X1	3hs	    3hs	    $300,00
Televisores X2	3hs	    6hs	    $400,00
disponibilidad	120 hs	180 hs	

CARGAR X1 DE LA FO Y X2 DE LA FO


navegar en la matriz Z,Z equivale a 0.0 y S3,s4 equivale a 3.6
*/

// cargar la tabla resultante
// 8columnas y 5 filas 
// elementos de tipo f32

//funcion objetivo
let fo_x1=-300.0;
let fo_x2=-400.0;
//                      z   x1    x2     s1    s2   r
let _linea_z:[f64;6]=[1.0, fo_x1, fo_x2, 0.0, 0.0, 0.0];

let _linea_montaje_s1_z=0.0;
// computadoras
let _linea_montaje_s1_x1=3.0;
// Televisores
let _linea_montaje_s1_x2=3.0;
//variable de holgura
let _linea_montaje_s1_s1=1.0;
let _linea_montaje_s1_s2=0.0;
//disponibilidad o _valor_resultado
let _linea_montaje_s1_r=120.0;

let _linea_montaje_s1:[f64;6]=[_linea_montaje_s1_z, 
_linea_montaje_s1_x1, _linea_montaje_s1_x2, 
_linea_montaje_s1_s1, _linea_montaje_s1_s2, 
_linea_montaje_s1_r];

//linea de acabado
let _linea_acabado_s2_z=0.0;
// computadoras
let _linea_acabado_s2_x1=3.0;
// Televisores
let _linea_acabado_s2_x2=6.0;
//variable de holgura
let _linea_acabado_s2_s1=0.0;
let _linea_acabado_s2_s2=1.0;
//disponibilidad o _valor_resultado
let _linea_acabado_s2_r=180.0;

let _linea_acabado_s2:[f64;6]=[_linea_acabado_s2_z, 
_linea_acabado_s2_x1, _linea_acabado_s2_x2, 
_linea_acabado_s2_s1, _linea_acabado_s2_s2, 
_linea_acabado_s2_r];

//mostrar la matriz
_mostrar_matriz(_linea_z, _linea_montaje_s1, _linea_acabado_s2);

let _col_piv=_buscar_el_menor_en_z(_linea_z);
println!("Columna Pivot en Z es {}", _col_piv);

// let _tabla_inicial:[f64;3]=[
//     _linea_z[f64;6], _linea_montaje_s1[f64;6], 
//     _linea_acabado_s2[f64;6]

// ];

// println!("{:?}", _tabla_inicial);




// let mut _tabla_inicial: [[f32; 6]; 3]=[
//     [1.0, -2.0, -3.0, 0.0, 0.0, 0.0],
//     [0.0, 1.0, 2.0, 1.0, 0.0, 8.0],
//     [0.0, 3.0, 1.0, 0.0, 1.0, 9.0],
//     ];
//     // [[tipo; cantidad columnas]; cantidad de filas]

//     _mostrar_matriz(_tabla_inicial);

    
    
    // let _col_piv:usize = _averiguar_columna_pivot(_tabla_inicial) as usize;
    // println!("la columna pivot es {}", _col_piv);

    // buscar en el resto de la matriz la fila pivot
    // 1..=fila-1
    // let _fil_piv=_averiguar_fila_pivot(_tabla_inicial, _col_piv);
    // println!("La fila pivot es {}", _fil_piv);

    
    // hacer gaussJordan

    // let _valor_nro_piv=_tabla_inicial[_fil_piv][_col_piv];

    //llamar a gauss_jordan
    //_Gauss_Jordan(_tabla_inicial, _valor_nro_piv, _fil_piv); //, _col_piv);

    //let nro_piv=_valor_nro_piv;
    // let fila_pivot=_fil_piv;
    //let colu_pivot=_col_piv;
    // let mut gausseando_un_nro=0.0;
    // let mut gausseando_un_nro;

    // for col in 1..5 {
    //     gausseando_un_nro=_tabla_inicial[fila_pivot][col]/_valor_nro_piv;
    //     _tabla_inicial[fila_pivot][col]=gausseando_un_nro;
    //     print!("{}, ", gausseando_un_nro);
    // }
    // println!("");


    // _mostrar_matriz(_tabla_inicial);


}

fn _mostrar_matriz(_z:[f64;6], _linea_montaje_s1:[f64;6], _linea_acabado_s2:[f64;6]) {
    
    println!("|     | {0: <5} | {1: <7} | {2: <7} | {3: <7} | {4: <7} | {5: <7} |",
    "Z","x1","x2", "S1", "S2", "R");
    println!("| Z   | {0: <5} | {1: <6} | {2: <6} | {3: <6} | {4: <6} | {5: <6} |", 
    _z[0],_z[1],_z[2],_z[3],_z[4],_z[5]);
    println!("| S1  | {0: <5} | {1: <6} | {2: <6} | {3: <6} | {4: <6} | {5: <6} |", 
    _linea_montaje_s1[0], _linea_montaje_s1[1], _linea_montaje_s1[2],
    _linea_montaje_s1[3], _linea_montaje_s1[4], _linea_montaje_s1[5]);
    println!("| S2  | {0: <5} | {1: <6} | {2: <6} | {3: <6} | {4: <6} | {5: <6} |", 
    _linea_acabado_s2[0], _linea_acabado_s2[1], _linea_acabado_s2[2],
    _linea_acabado_s2[3], _linea_acabado_s2[4], _linea_acabado_s2[5]);
    println!("");
    
}

fn _buscar_el_menor_en_z(_z:[f64;6]) -> f64 {
    let mut _menor = _z[0];
    let mut _indice=0;
    println!("_menor {}", _menor);
    for (_i, _menor_en_z) in _z.iter().enumerate() {
        //para habilitar el indice (_i) se debe agregar para su ubicación
        //matriz.iter().enumerate() .... iter es para el bucle enumerate para
        //enumerar el indice, el indice debe ir primero en el for (indice, otra cosa)
        //println!("_menor_en_z: {}", _menor_en_z);
        
        //print!("menorENz {} es menor que menor {}, entonces ",_menor_en_z, _menor );
        if _menor_en_z < &_menor {
            // _menor = _menor_en_z;
            
            println!("{} < {}", _menor_en_z, _menor );
            _menor = *_menor_en_z;
            _indice=_i;
        }
        

    }
    println!("El indice es {}", _indice);
    _menor
}


// fn _mostrar_matriz(_m:[[f32; 6]; 3]) {
//     println!("**************************************************");
//     println!("*        Z     x1     x2     S1     S2     R     *");
//     println!("**************************************************");
//     println!("* Z      {}     {}     {}     {}      {}      {}     *",
//     _m[0][0],_m[0][1],_m[0][2],_m[0][3],_m[0][4],_m[0][5]);
//     println!("* S1     {}      {}      {}     {}      {}      {}     *",
//     _m[1][0],_m[1][1],_m[1][2],_m[1][3],_m[1][4],_m[1][5]);
//     println!("* S2     {}      {}      {}     {}      {}      {}     *",
//     _m[2][0],_m[2][1],_m[2][2],_m[2][3],_m[2][4],_m[2][5]);
//     println!("**************************************************");
// }

// fn _averiguar_columna_pivot(_m:[[f32; 6]; 3]) -> i32 {
// averiguar la columna pivot




// let mut _obtenido:f32 =0.0;
// let mut _valor_menor: f32=0.0;
// let mut _fila:i32=0;
// let mut _columna_pivot:i32=0;
// let mut _contar_col:i32=0;
// for menor in _m[0].iter() {
//     _obtenido=*menor;
//     if _obtenido < _valor_menor {
//         _valor_menor=_obtenido;
//         _columna_pivot=_contar_col;

//     }
//     _contar_col+=1;
// }


// println!("el valor menor de la fila {} es: {}", _fila, _valor_menor);
// _columna_pivot
// }

// fn _averiguar_fila_pivot(_m:[[f32; 6]; 3], _c:usize) -> usize { //columnas y filas
//     let mut _fila_pivot:usize=0;
//     let mut _columna_pivot=_c;
//     let _columna_resultado:usize=5;
//     let mut _fila_control_anterior=0.0;
//     let mut _fila_control_posterior=0.0;
//     let mut _valor_resultado=0.0;
//     let mut _valor_fila_pivot=0.0;
//     let mut _primera_iteracion=true;


//     for _fila_recorrida in 1..3 {
//         _valor_resultado=_m[_fila_recorrida][5];
//         println!("Fila: {}, valor de la columna R: {}",_fila_recorrida, _valor_resultado);
//         _valor_fila_pivot=_m[_fila_recorrida][_columna_pivot];
//         println!("Fila: {}, valor de la columna Pivot: {}",_fila_recorrida, _valor_fila_pivot);
//         if _valor_fila_pivot > 0.0 {
//             _fila_control_posterior=_valor_resultado / _valor_fila_pivot;
//             println!("Fila: {}, coeficiente R/CP: {}",_fila_recorrida, _fila_control_posterior);
//             println!("fila control anterior {} y fila control posterior {}",_fila_control_anterior, _fila_control_posterior);
//         }
       

//         if _fila_control_posterior < _fila_control_anterior {
//             _fila_control_anterior=_fila_control_posterior;
//             _fila_pivot=_fila_recorrida;
//         }
//         if _primera_iteracion {
//             _fila_control_anterior=_fila_control_posterior;
//             _fila_pivot=_fila_recorrida;
//             _primera_iteracion=false;
//         }
//     }

    // recorrido de toda la matriz
    // for fila in 0..5 { //filas
    //     for columna in 0..8 { //columnas
    //         println!("fila: {}, columna: {} - Dato: {}", fila, columna, _m[fila][columna]);
    //     }
    // }
    
    // if _fila_pivot==1 {
    //     println!("el valor menor de la fila S1 es: {}", _fila_control_posterior);
    // } else if _fila_pivot==2 {
    //     println!("el valor menor de la fila S2 es: {}", _fila_control_posterior);
    // } else if _fila_pivot==3 {
    //     println!("el valor menor de la fila S3 es: {}", _fila_control_posterior);
    // }else if _fila_pivot==4 {
    //     println!("el valor menor de la fila S4 es: {}", _fila_control_posterior);
    // }

    // _fila_pivot

// }

/*fn _Gauss_Jordan(_m:[[f32; 8]; 5], _valor_nro_piv, _fil_piv){ //, _col_piv){
    let nro_pivot=_valor_nro_piv;
    let fila_pivot=_fil_piv;
    //let colu_pivot=_col_piv;
    let mut gausseando_un_nro=0.0;
    for col in 1..8 {
        gausseando_un_nro=_m[fila_pivot][col]/_valor_nro_piv;
        _m[fila_pivot][col]=gausseando_un_nro;
    }


 /* recorrido de toda la matriz
    for fila in 0..5 { //filas
        for columna in 0..8 { //columnas
            println!("fila: {}, columna: {} - Dato: {}", fila, columna, _m[fila][columna]);
        }
    }
    */



}*/

