import { ClientOptions, ClientProviderOptions, Transport } from '@nestjs/microservices';
import { join } from 'path';


export const rustMicroserviceOptions: ClientOptions = {
  transport: Transport.GRPC,
  options: {
    url: "127.0.0.1:50051",
    package: 'rust_app',
    protoPath: join(__dirname, '../src/proto/user_service.proto'),
  },
};
