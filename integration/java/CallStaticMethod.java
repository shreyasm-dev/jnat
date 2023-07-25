public class CallStaticMethod {
  private static native void caller();

  static {
    System.loadLibrary("call_static_method");
  }

  public static void main(String[] args) {
    CallStaticMethod.caller();
  }

  public static void callback(int n, String s) {
    System.out.print("Static callback: " + n + s + "\n");
  }
}
