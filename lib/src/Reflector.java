import java.lang.reflect.Method;

// Reflector class is a helper class used to directly interact with Rust
// without any other hacking code from Rust and prevent lots of 
// unnesccesary complexity.
//
// Reflector will only be used when "interop" features is on, and
// will be built in build.rs, this means the build environment should be
// based on your machine instead of pre-compiled binary file being shared
// around.
public class Reflector {
    public static Class<?> forName(String name) throws ClassNotFoundException {
        return Class.forName(name);
    }
    
    // Class<?> interface
    
    public static String getName(Class<?> clazz) {
        return clazz.getName();
    }
    
    public static String getCanonicalName(Class<?> clazz) {
        return clazz.getCanonicalName();
    }
    
    public static int getModifiers(Class<?> clazz) {
        return clazz.getModifiers();
    }
    
    public static Method[] getDeclaredMethods(Class<?> clazz) {
        return clazz.getDeclaredMethods();
    }
    
    // Method interface
    
    public static String getName(Method method) {
        return method.getName();
    }
}