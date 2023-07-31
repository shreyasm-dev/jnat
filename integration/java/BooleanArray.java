public class BooleanArray {
  private static native void method();

  static {
    System.loadLibrary("boolean_array");
  }

  public static void main(String[] args) {
    BooleanArray.method();
  }
}
