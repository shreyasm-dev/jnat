public class Hello {
  private static native String hello(String name);

  static {
    System.loadLibrary("hello");
  }

  public static void main(String[] args) {
    System.out.print(Hello.hello("world") + "\n");
  }
}
