public class CallMethod {
  private static native void caller(CallMethod obj);

  static {
    System.loadLibrary("call_method");
  }

  public static void main(String[] args) {
    CallMethod.caller(new CallMethod());
  }

  public void callback(int n, String s) {
    System.out.print("Static callback: " + n + s + "\n");
  }
}
