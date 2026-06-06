package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ICU4XMeasureUnitLib: Library {
    fun icu4x_ICU4XMeasureUnit_destroy_mv1(handle: Pointer)
    fun icu4x_ICU4XMeasureUnit_deserialize_mv1(s: Slice): ResultPointerInt
}

class ICU4XMeasureUnit internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class ICU4XMeasureUnitCleaner(val handle: Pointer, val lib: ICU4XMeasureUnitLib) : Runnable {
        override fun run() {
            lib.icu4x_ICU4XMeasureUnit_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, ICU4XMeasureUnit.ICU4XMeasureUnitCleaner(handle, ICU4XMeasureUnit.lib));
    }

    companion object {
        internal val libClass: Class<ICU4XMeasureUnitLib> = ICU4XMeasureUnitLib::class.java
        internal val lib: ICU4XMeasureUnitLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Parse a [ICU4XMeasureUnit] from a string.
        */
        fun deserialize(s: String): Result<ICU4XMeasureUnit> {
            val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
            
            val returnVal = lib.icu4x_ICU4XMeasureUnit_deserialize_mv1(sSliceMemory.slice);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal 
                    val returnOpaque = ICU4XMeasureUnit(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                sSliceMemory.close()
            }
        }
    }

}