public class StaticField {
  private static native void method();

  public static int staticField = 0;

  static {
    System.loadLibrary("static_field");
  }

  public static void main(String[] args) {
    StaticField.method();
  }
}
