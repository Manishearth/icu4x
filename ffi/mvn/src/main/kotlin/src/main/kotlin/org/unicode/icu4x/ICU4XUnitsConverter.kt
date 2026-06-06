package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ICU4XUnitsConverterLib: Library {
    fun icu4x_ICU4XUnitsConverter_destroy_mv1(handle: Pointer)
    fun icu4x_ICU4XUnitsConverter_convert_mv1(handle: Pointer, value: Double): Double
}

class ICU4XUnitsConverter internal constructor (
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

    private class ICU4XUnitsConverterCleaner(val handle: Pointer, val lib: ICU4XUnitsConverterLib) : Runnable {
        override fun run() {
            lib.icu4x_ICU4XUnitsConverter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, ICU4XUnitsConverter.ICU4XUnitsConverterCleaner(handle, ICU4XUnitsConverter.lib));
    }

    companion object {
        internal val libClass: Class<ICU4XUnitsConverterLib> = ICU4XUnitsConverterLib::class.java
        internal val lib: ICU4XUnitsConverterLib = Native.load("icu4x", libClass)
    }
    
    /** Convert a value.
    */
    fun convert(value: Double): Double {
        
        val returnVal = lib.icu4x_ICU4XUnitsConverter_convert_mv1(handle, value);
        return (returnVal)
    }

}