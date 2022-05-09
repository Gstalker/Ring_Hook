package bin.gstalker.ring.test;

import androidx.appcompat.app.AppCompatActivity;

import android.content.Context;
import android.os.Bundle;
import android.util.Log;
import android.view.View;

public class MainActivity extends AppCompatActivity {

    private static Thread daemon;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        Log.wtf("GSTALKER", "test Start!");
    }

    public void onTag(View view) {
        try{
            Gstalker.start_test("hello_world from MainActivity out of thread");
        }
        catch( Throwable t ){
            Log.wtf("GSTALKER", "test End");
        }
    }
}