import { Controller, Get, OnModuleInit, Param, Res, UseFilters } from '@nestjs/common';
import { Client, ClientGrpc } from '@nestjs/microservices';
import { Response } from "express";
import { timeout } from "rxjs/operators";
import { IGrpcRustUserService } from '../grpc.interface';
import { rustMicroserviceOptions } from '../grpc.options';

const FIVE_SECONDS = 5000;

@Controller()
export class UserController implements OnModuleInit {
  public constructor() { }

  @Client(rustMicroserviceOptions)
  private rustClient: ClientGrpc;
  private grpcRustService: IGrpcRustUserService;

  onModuleInit() {
    this.grpcRustService = this.rustClient.getService<IGrpcRustUserService>('UserService');
  }

  @Get('user')
  public async findAll(@Res() res: Response): Promise<void> {
    const response$ = this.grpcRustService.findMany({});
    response$.pipe(timeout(FIVE_SECONDS)).subscribe(
      (value) => res.send(value),
      (error) => console.error(error),
      () => console.info('Microservice rust request completed')
    );
  }

  @Get('user/:idUser')
  public async findUserById(@Param("idUser") idUser: number, @Res() res: Response): Promise<void> {
    const response$ = this.grpcRustService.findOne({ id: idUser });
    response$.pipe(timeout(FIVE_SECONDS)).subscribe(
      (value) => res.send(value),
      (error) => res.send(error),
      () => console.info('Microservice rust request completed')
    );
  }
}
