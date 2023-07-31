public class Array {
  private static native void method();

  static {
    System.loadLibrary("array");
  }

  public static void main(String[] args) {
    Array.method();
  }
}
