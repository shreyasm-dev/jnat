public class StaticMethod {
  private static native void caller();

  static {
    System.loadLibrary("static_method");
  }

  public static void main(String[] args) {
    StaticMethod.caller();
  }

  public static void callback(int n, String s) {
    System.out.print("Static callback: " + n + s + "\n");
  }
}
