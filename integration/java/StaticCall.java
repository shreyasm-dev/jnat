public class StaticCall {
  private static native void caller();

  static {
    System.loadLibrary("static_call");
  }

  public static void main(String[] args) {
    StaticCall.caller();
  }

  public static void callback() {
    System.out.println("Static callback");
  }
}
