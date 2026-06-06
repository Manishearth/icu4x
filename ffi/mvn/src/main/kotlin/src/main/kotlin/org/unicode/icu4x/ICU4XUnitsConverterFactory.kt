package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ICU4XUnitsConverterFactoryLib: Library {
    fun icu4x_ICU4XUnitsConverterFactory_destroy_mv1(handle: Pointer)
    fun icu4x_ICU4XUnitsConverterFactory_create_mv1(): Pointer
    fun icu4x_ICU4XUnitsConverterFactory_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_ICU4XUnitsConverterFactory_converter_mv1(handle: Pointer, inputUnit: Pointer, outputUnit: Pointer): Pointer?
}

class ICU4XUnitsConverterFactory internal constructor (
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

    private class ICU4XUnitsConverterFactoryCleaner(val handle: Pointer, val lib: ICU4XUnitsConverterFactoryLib) : Runnable {
        override fun run() {
            lib.icu4x_ICU4XUnitsConverterFactory_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, ICU4XUnitsConverterFactory.ICU4XUnitsConverterFactoryCleaner(handle, ICU4XUnitsConverterFactory.lib));
    }

    companion object {
        internal val libClass: Class<ICU4XUnitsConverterFactoryLib> = ICU4XUnitsConverterFactoryLib::class.java
        internal val lib: ICU4XUnitsConverterFactoryLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a new [ICU4XUnitsConverterFactory] with compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.2.0/icu/units/converter_factory/struct.ConverterFactory.html#method.new) for more information.
        */
        fun create(): ICU4XUnitsConverterFactory {
            
            val returnVal = lib.icu4x_ICU4XUnitsConverterFactory_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = ICU4XUnitsConverterFactory(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a new [ICU4XUnitsConverterFactory] with a data provider.
        *
        *See the [Rust documentation for `try_new_with_buffer_provider`](https://docs.rs/icu/2.2.0/icu/units/converter_factory/struct.ConverterFactory.html#method.try_new_with_buffer_provider) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<ICU4XUnitsConverterFactory> {
            
            val returnVal = lib.icu4x_ICU4XUnitsConverterFactory_create_with_provider_mv1(provider.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ICU4XUnitsConverterFactory(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /** Create a new [ICU4XUnitsConverter] for the given input and output units.
    */
    fun converter(inputUnit: ICU4XMeasureUnit, outputUnit: ICU4XMeasureUnit): ICU4XUnitsConverter? {
        
        val returnVal = lib.icu4x_ICU4XUnitsConverterFactory_converter_mv1(handle, inputUnit.handle, outputUnit.handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal ?: return null
        val returnOpaque = ICU4XUnitsConverter(handle, selfEdges, true)
        return returnOpaque
    }

}