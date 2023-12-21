cd(@__DIR__)

mutable struct Value
    val
    ref
end

Value(val)=begin
    obj=Value(val, nothing)
    obj.ref=Ref(obj.val)|>pointer_from_objref
    return obj
end

mutable struct ValueFFI
    tag::Cint
    val::Ptr{Cvoid}
end

test_val=Value(UInt8(26))
test_val_ffi=ValueFFI(0, test_val.ref)
@ccall "../target/debug/librs_jl_test.so".struct_from_julia(pointer_from_objref(test_val_ffi)::Ptr{ValueFFI})::Cvoid

mutable struct ValueEnum
    tag::Cint
    val::Ptr{Cvoid}
end

test_val=Value(Int32(36))
test_val_enum_null=ValueEnum(0, Ptr{Cvoid}())
test_val_int=ValueEnum(1, test_val.ref)
@ccall "../target/debug/librs_jl_test.so".enum_from_julia(pointer_from_objref(test_val_enum_null)::Ptr{ValueEnum})::Cvoid
@ccall "../target/debug/librs_jl_test.so".enum_from_julia(pointer_from_objref(test_val_int)::Ptr{ValueEnum})::Cvoid

# Conclusion: Julia types can be converted into Rust's 1) structs, in the same order of fields or 2)enums, with the first field being the enum's index.

mutable struct BoxEnum
    tag::Cint
    val::Ptr{Cvoid}
end

test_val=Value(Int32(120))
test_val_int=BoxEnum(1, test_val.ref)
@ccall "../target/debug/librs_jl_test.so".conv_to_box(pointer_from_objref(test_val_int)::Ptr{BoxEnum})::Cvoid
#It does work, but since it was converted to Box type, the memory got deallocated write after its use. The remaining pointer is a dangling pointer. <-X
# Sorry, I was wrong. It's not deallocated. It's just the difference between running via vscode extension and running on repl.
reinterpret(Ptr{Int32}, test_val.ref)|>unsafe_load

#= It is considered a double free or corruption.
@ccall "../target/debug/librs_jl_test".drop_boxenum(pointer_from_objref(test_val_int)::Ptr{BoxEnum})::Cvoid
=#
rval_ptr=@ccall "../target/debug/librs_jl_test".get_boxenum()::Ptr{BoxEnum}
rval=rval_ptr|>unsafe_load
reinterpret(Ptr{Int32}, rval.val)|>unsafe_load|>println
@ccall "../target/debug/librs_jl_test".drop_boxenum(rval_ptr::Ptr{BoxEnum})::Cvoid
#Rust value is now dropped, and the remaining pointer is a dangling pointer.
reinterpret(Ptr{Int32}, rval.val)|>unsafe_load|>println




