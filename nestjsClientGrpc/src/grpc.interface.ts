import { Observable } from 'rxjs';

export interface IGrpcRustUserService {
  findOne(ByIdRequest: IByIdRequest): Observable<IFindUserResponse>;
  findMany(number: IEmptyRequest): Observable<IFindAllUsersResponse>;
}

interface IEmptyRequest { }
interface IByIdRequest {
  id: number;
}
interface IFindAllUsersResponse {
  users: IFindUserResponse[];
}
interface IFindUserResponse {
  id: number;
  email: string;
  username: string;
}
interface ICreateRequest {
  email: string;
  username: string;
  password: string;
}
interface ICreateResponse {
  id: number;
}