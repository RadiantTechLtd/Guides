from mpi4py import MPI


comm = MPI.COMM_WORLD
rank = comm.Get_rank()

if rank == 0:
    data = {"name": "freddy", "colour": "green"}
    comm.send(data, dest=1)
    print(f"On process 0, data sent.")
elif rank == 1:
    data = comm.recv(source=0)
    print(f"On process 1, data is {data}.")
else:
    print(f"I am process {rank} and I do not have any data.")
