import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

@Target(ElementType.TYPE)
@Retention(RetentionPolicy.CLASS)
public @interface VisibleAnnotation {
    byte b() default 1;
    short s() default 1;
    int i() default 1;
    float f() default 1;
    long j() default 1;
    double d() default 1;
    String string() default "KEK";
    Enum e() default Enum.KAPI;
}
