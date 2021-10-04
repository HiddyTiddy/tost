!! this is a comment
!! sample tost program

charred message "Hello Tost"

!! happy function: doesnt return anything, has {:  :}
!! sad function: returns Toast that's so sad :{  }:
toaster func_name <Bread arg> -> Toast :{
    ding new iToast(10);
}:

toaster main<> {:
    serve(message);
    serve(func_name());
:}
