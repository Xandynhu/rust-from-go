module module_with_ports(
    input logic clk,
    input logic rst_n,
    input logic [7:0] data_in,
    output logic [7:0] data_out,
    inout logic bidir_port,
    output logic [3:0] multi_bit_out,
    input logic [3:0] multi_bit_in
);

    // Define your logic here
    // Example:
    always_ff @(posedge clk or negedge rst_n)
    begin
        if (!rst_n)
            data_out <= 8'h00;
        else
            data_out <= data_in;
    end

endmodule
