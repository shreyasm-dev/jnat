public class Method {
  private static native void caller(Method obj);

  static {
    System.loadLibrary("method");
  }

  public static void main(String[] args) {
    Method.caller(new Method());
  }

  public void callback(int n, String s) {
    System.out.print("Static callback: " + n + s + "\n");
  }
}
