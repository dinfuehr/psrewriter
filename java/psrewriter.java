import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileReader;
import java.io.FileWriter;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;

public class psrewriter {
	public static void main(String[] args) {
		File in = new File(args[0]);
		File out = new File(args[1]);

		try(BufferedReader br = new BufferedReader(new FileReader(in))) {
			BufferedWriter bw = new BufferedWriter(new FileWriter(out));
				boolean inside = false;
				String line;

				while( (line = br.readLine()) != null ) {
					String find = inside ? ">>" : "<<";

					if(line.contains(find)) { inside = !inside; }

					bw.write(line);
					if(!inside) { bw.newLine(); }
				}

		    bw.close();
		    br.close();
		} catch(Exception e) {
			e.printStackTrace();
		}
	}
}
