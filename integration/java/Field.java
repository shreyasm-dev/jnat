public class Field {
  private static native void method(Field obj);

  public int field = 0;

  static {
    System.loadLibrary("field");
  }

  public static void main(String[] args) {
    Field.method(new Field());
  }
}
