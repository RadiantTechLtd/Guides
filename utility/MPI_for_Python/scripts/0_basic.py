from mpi4py import MPI

comm = MPI.COMM_WORLD
name = MPI.Get_processor_name()

print(f"Process name: {name}. Rank: {comm.rank}")
