public class ObjectArray {
  private static native void method();

  static {
    System.loadLibrary("object_array");
  }

  public static void main(String[] args) {
    ObjectArray.method();
  }
}
