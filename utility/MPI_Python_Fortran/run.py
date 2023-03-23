from mpi4py import MPI
import dill
import time

MPI.pickle.__init__(dill.dumps, dill.loads)


comm = MPI.COMM_WORLD
rank = comm.Get_rank()
size = comm.Get_size()
name = MPI.Get_processor_name()

if rank == 0:
    data = 777
    comm.send(data, dest=1)

    time.sleep(2)
    print("READY")
    comm.send([data, MPI.INT], dest=3, tag=0)  # TODO: Can't pickle MPI.INT type :(
    print(f"Python process 0 of {size}, data sent.")
elif rank == 1:
    data = comm.recv(source=0)
    print(f"Python process 1 of {size}, data received: {data}.")
else:
    print(f"I am process {rank} and I do not have any data.")
