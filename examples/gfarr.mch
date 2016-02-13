MACHINE
    /* Generic Fixed Array */
    gfarr(VALUE, default_val, capacity)
    
CONSTRAINTS
    default_val : VALUE &
    capacity : NAT1
    
CONCRETE_VARIABLES
    arr
    
INVARIANT
    arr : 1..capacity --> VALUE
    
INITIALISATION
    arr := (1..capacity) × {default_val}
    
OPERATIONS
    val <-- get(ind) =
    PRE
        ind : dom(arr)
    THEN
        val := arr(ind)
    END;
    
    set(ind, val) =
    PRE
        ind : dom(arr) &
        val : VALUE
    THEN
        arr(ind) := val
    END;
    
    shift_right =
    BEGIN
        arr := (id({1}) \/ %xx.( xx : 2..capacity | xx - 1 ) ◦ arr)
    END;
    
    shift_left =
    BEGIN
        arr := (%xx.( xx : 1..capacity-1 | xx + 1 ) \/ id({capacity}) ◦ arr)
    END    
    
END
