import java.lang.reflect.Method;

// Reflector class is a helper class used to directly interact with Rust
// without any other hacking code from Rust and prevent lots of 
// unnesccesary complexity.
//
// Reflector will only be used when "interop" features is on, and
// will be built in build.rs, this means the build environment should
// based on your machine instead of pre-compiled binary file being shared
// around.
public class Reflector {
    public static Class<?> forName(String name) {
        return Class.forName(name);
    }
    
    public static String getName(Class<?> clazz) {
        return clazz.getName();
    }
    
    public static int getModifiers(Class<?> clazz) {
        return clazz.getModifiers();
    }
    
    public static Method[] getDeclaredMethods(Class<?> clazz) {
        return clazz.getgetDeclaredMethods();
    }
}